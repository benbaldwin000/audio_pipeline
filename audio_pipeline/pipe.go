package audio_pipeline

type AudioPipe interface {
	init() error
	pipe(chan uint8, chan uint8) error
	purge() error
}
