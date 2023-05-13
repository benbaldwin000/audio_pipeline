package audio_pipeline

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
}

func (pipeline AudioPipeline) Init() error {
	return nil
}

func (pipeline *AudioPipeline) Queue(id string) error {
	return nil
}

func (pipeline *AudioPipeline) Purge() error {
	return nil
}
