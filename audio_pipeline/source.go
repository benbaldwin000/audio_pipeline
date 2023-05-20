package audio_pipeline

type AudioSource interface {
	Init() error
	Get(query *AudioSourceQuery) (*Audio, error)
}

type AudioSourceQuery struct {
	Id         *string
	SearchTerm *string
	Sources    *[]string
}
