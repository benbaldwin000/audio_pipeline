fn main() {}

use std::{sync::{
    mpsc::{sync_channel, Receiver, SendError, SyncSender, TryRecvError},
    Arc,
}, thread::{self, JoinHandle}};

type Sample = f32;

struct StaticAudioSource<const N: usize> {
    data: [Sample; N],
}

struct ContinuousAudioSource {
    sender: SyncSender<Sample>,
    reciever: Receiver<Sample>,
}

trait AudioPipe: Sync {
    fn pipe(
        &self,
        ctx: &TransformCtx,
        from: Receiver<Sample>,
        to: SyncSender<Sample>,
    ) -> Result<(), String>;
}

trait AudioFilter: Sync {
    fn transform(&self, ctx: &TransformCtx, data: &mut [Sample]) -> Result<(), String>;
}

impl AudioPipe for dyn AudioFilter {
    fn pipe(
        &self,
        ctx: &TransformCtx,
        from: Receiver<Sample>,
        to: SyncSender<Sample>,
    ) -> Result<(), String> {
        let mut recv_result: Result<Sample, TryRecvError>;
        let mut send_result: Result<(), SendError<Sample>>;
        let mut sample_window = Vec::<Sample>::with_capacity(ctx.window_size);
        let mut transform_result: Result<(), String>;

        loop {
            let mut i = 0;
            while i < ctx.window_size {
                recv_result = from.try_recv();
                if let Err(err) = recv_result {
                    match err {
                        TryRecvError::Empty => break,
                        TryRecvError::Disconnected => {
                            return Err(format!("error recieving sample: {err}"))
                        }
                    }
                }

                sample_window.push(recv_result.unwrap());
                i += 1;
            }

            transform_result = self.transform(ctx, &mut sample_window[0..(i - 1)]);
            if let Err(err) = transform_result {
                return Err(format!("error transforming sample window: {err}"));
            }

            for sample in sample_window.iter() {
                send_result = to.send(*sample);
                if let Err(err) = send_result {
                    return Err(format!("error sending result: {err}"));
                }
            }

            sample_window.truncate(0);
        }
    }
}

impl AudioFilter for dyn AudioPipe {
    fn transform(&self, ctx: &TransformCtx, data: &mut [Sample]) -> Result<(), String> {
        let sample_count = data.len();
        let (sender, reciever) = sync_channel(sample_count);
        for sample in data.iter() {
            if let Err(err) = sender.send(sample.clone()) {
                return Err(format!("error sending sample: {err}"));
            }
        }

        for i in 0..sample_count {
            match reciever.recv() {
                Ok(transformed) => data[i] = transformed,
                Err(err) => return Err(format!("error recieving sample {err}")),
            }
        }

        Ok(())
    }
}

struct TransformCtx {
    window_size: usize,
    fitting_buffer: usize
}

struct AudioTransformation<const N: usize> {
    source: StaticAudioSource<N>,
    ctx: TransformCtx,
    transformations: Vec<Arc<dyn AudioFilter>>,
    transform_cursor: usize,
    read_cursor: usize,
}

impl<const N: usize> AudioTransformation<N> {
    fn new(
        ctx: TransformCtx,
        source: StaticAudioSource<N>,
        transformations: &[&Arc<dyn AudioFilter>],
    ) -> Self {
        Self {
            source,
            ctx,
            transformations: transformations
                .into_iter()
                .map(|arc| (*arc).clone())
                .collect(),
            transform_cursor: 0,
            read_cursor: 0,
        }
    }

    fn transform(&mut self) -> Result<(), String> {
        for transformation in self.transformations.iter() {
            if let Err(err) = transformation.transform(&self.ctx, &mut self.source.data) {
                return Err(format!("error transforming source: {err}"));
            }
        }

        self.transform_cursor = self.source.data.len();
        Ok(())
    }
}

struct AudioPipeline {
    ctx: Arc<TransformCtx>,
    pipes: Vec<Arc<dyn AudioPipe>>,
}

impl AudioPipeline {
    fn new(
        ctx: TransformCtx,
        pipes: &[&Arc<dyn AudioPipe>],
    ) -> Self {
        Self {
            ctx: Arc::new(ctx),
            pipes: pipes.into_iter().map(|arc| (*arc).clone()).collect(),
        }
    }

    fn pipe(&self, from: Receiver<Sample>, to: SyncSender<Sample>) -> Result<Vec<JoinHandle<()>>, String> {
        let mut next_reciever = from;
        let mut reciever: Receiver<Sample>;
        let mut sender: SyncSender<Sample>;

        let pipe_count = self.pipes.len();
        let mut handles = Vec::with_capacity(pipe_count);

        let mut ctx_arc: Arc<TransformCtx>;
        let mut pipe_arc: Arc<dyn AudioPipe>;
        for (i, pipe) in self.pipes.iter().enumerate() {
            ctx_arc = self.ctx.clone();
            pipe_arc = (*pipe).clone();

            reciever = next_reciever;
            (sender, next_reciever) = sync_channel(ctx_arc.fitting_buffer);

            // handles.push(thread::spawn(move || {
            //     pipe_arc.pipe(&(*ctx_arc), from, to);
            // }));
        }

        Ok(handles)
    }
}
