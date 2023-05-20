package audio_pipeline

import "fmt"

type PipelineConfig struct {
	PreLoad    uint32
	PreProcess uint32
}

type AudioPipeline struct {
	Config  PipelineConfig
	Sources map[string]*AudioSource
	Codecs  map[string]*AudioCodec
	Pipes   []*AudioPipe
	Drains  map[string]*AudioDrain

	queryQueue []*AudioSourceQuery
	queue      []*Audio
	channels   []*chan uint8
}

func (pipeline AudioPipeline) Init() (err error) {
	if pipeline.Sources == nil ||
		pipeline.Codecs == nil ||
		pipeline.Pipes == nil ||
		pipeline.Drains == nil {
		return fmt.Errorf("Incomplete pipeline")
	}

	pipeline.queryQueue = make([]*AudioSourceQuery, 0)
	pipeline.queue = make([]*Audio, pipeline.Config.PreLoad)
	pipeline.channels = make([]*chan uint8, len(pipeline.Pipes)+1)

	for id, source := range pipeline.Sources {
		err = (*source).Init()
		if err != nil {
			return fmt.Errorf("Error initializing source \"%v\": %v", id, err)
		}
	}

	in := make(chan uint8)
	pipeline.channels[0] = &in
	for i, pipe := range pipeline.Pipes {
		err = (*pipe).Init()
		if err != nil {
			return fmt.Errorf("Error initializing pipe at index %v: %v", i, err)
		}

		out := make(chan uint8)
		pipeline.channels[i+1] = &out
	}

	for id, drain := range pipeline.Drains {
		err = (*drain).Init()
		if err != nil {
			return fmt.Errorf("Error initializing drain \"%v\": %v", id, err)
		}
	}

	return nil
}

func (pipeline *AudioPipeline) Run() error {
	var in, out *chan uint8
	for i, pipe := range pipeline.Pipes {
		in = pipeline.channels[i]
		out = pipeline.channels[i+1]
		go (*pipe).Pipe(*in, *out)
	}

	return nil
}

func (pipeline *AudioPipeline) Queue(id string) error {
	return nil
}

func (pipeline *AudioPipeline) Purge() (err error) {
	pipeline.queryQueue = make([]*AudioSourceQuery, 0)
	for i := range pipeline.queue {
		pipeline.queue[i] = nil
	}

	for i, pipe := range pipeline.Pipes {
		err = (*pipe).Purge()
		if err != nil {
			return fmt.Errorf("Error purging pipe at index %v: %v", i, err)
		}
	}

	for id, drain := range pipeline.Drains {
		err = (*drain).Purge()
		if err != nil {
			return fmt.Errorf("Error purging drain \"%v\": %v", id, err)
		}
	}

	return nil
}
