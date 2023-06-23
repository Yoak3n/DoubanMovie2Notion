package model

type Title struct {
	Type string `json:"type"`
	Text struct {
		Content string `json:"content"`
	} `json:"text"`
}
type Multi struct {
	Name string `json:"name"`
}

type Rich struct {
	Type string `json:"type"`
	Text Text   `json:"text"`
}

type Text struct {
	Content string `json:"content"`
}

type File struct {
	Name     string   `json:"name"`
	Type     string   `json:"type"`
	External External `json:"external"`
}
type External struct {
	Url string `json:"url"`
}
