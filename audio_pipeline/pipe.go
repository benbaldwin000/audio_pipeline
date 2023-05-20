package audio_pipeline

type AudioPipe interface {
	Init() error
	Pipe(<-chan uint8, chan<- uint8)
	Purge() error
}
