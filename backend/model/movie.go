package model

type DoubanMovie struct {
	Name        string
	Director    [5]string
	Actor       [5]string
	ReleaseTime string
	Writer      [5]string
	Score       float32
	Genre       [5]string
	Region      [5]string
	Duration    int
	Year        uint
	Imdb        string
	Language    [5]string
	RankNo      string
	Src         string
}

type Data struct {
	Properties `json:"properties"`
	Parent     `json:"parent"`
}

type Parent struct {
	Type       string `json:"type"`
	DatabaseID string `json:"database_id"`
}

type Properties struct {
	Movie    `json:"Movie"`
	Director `json:"Director"`
	Actor    `json:"Actor"`
	UpDate   `json:"UpDate"`
	Region   `json:"Region"`
	Writer   `json:"Writer"`
	Genre    `json:"Genre"`
	Cover    `json:"Cover"`
	Language `json:"Language"`
	Year     `json:"Year"`
	Imdb     `json:"imdb"`
	Duration `json:"Duration"`
	Rank     `json:"Rank,omitempty"`
	Score    `json:"Score,omitempty"`
}

type Movie struct {
	Title []Title `json:"title"`
}

type Director struct {
	MultiSelect []Multi `json:"multi_select"`
}
type Actor struct {
	MultiSelect []Multi `json:"multi_select"`
}

type UpDate struct {
	RichText []Rich `json:"rich_text"`
}

type Score struct {
	Number float32 `json:"number"`
}
type Region struct {
	MultiSelect []Multi `json:"multi_select"`
}
type Writer struct {
	MultiSelect []Multi `json:"multi_select"`
}

type Genre struct {
	MultiSelect []Multi `json:"multi_select"`
}

type Cover struct {
	Files []File `json:"files"`
}
type Language struct {
	MultiSelect []Multi `json:"multi_select"`
}

type Year struct {
	Number int `json:"number"`
}
type Duration struct {
	Number int `json:"number"`
}

type Rank struct {
	Number int64 `json:"number"`
}
type Imdb struct {
	RichText []Rich `json:"rich_text"`
}
