package audio_pipeline

type AudioSource interface {
	get(id string) (Audio, error)
}
