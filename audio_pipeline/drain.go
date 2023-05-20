package audio_pipeline

type AudioDrain interface {
	Init() error
	Drain(<-chan uint8)
	Purge() error
}
