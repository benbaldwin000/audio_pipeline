package audio_pipeline

type AudioCodec interface {
	Encode(in chan uint8, out chan uint8) error
	Decode(in chan uint8, out chan uint8) error
}
