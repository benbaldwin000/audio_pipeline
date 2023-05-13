package audio_pipeline

type Audio struct {
	Encoding   string
	Channels   uint16
	SampleRate uint32
	BitDepth   uint16
	Samples    chan uint8
}
