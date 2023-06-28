package network

import (
	"bytes"
	"douban_movie/backend/model"
	"douban_movie/config"
	"encoding/json"
	"log"
	"net/http"
	"strconv"
)

var movie *model.DoubanMovie
var data *model.Data

func initBody(m *model.DoubanMovie) {
	movie = m
	data = genBody()
}

func MakePost(movie *model.DoubanMovie) *http.Request {
	initBody(movie)

	b, _ := json.Marshal(data)
	buf := bytes.NewBuffer(b)
	req, err := http.NewRequest("POST", "https://api.notion.com/v1/pages/", buf)
	if err != nil {
		log.Println(err)
	}
	req.Header.Set("Accept", "application/json")
	req.Header.Set("Notion-Version", "2022-06-28")
	req.Header.Set("Content-Type", "application/json")
	req.Header.Set("Authorization", "Bearer "+config.Conf.Token)
	return req
}

func genBody() *model.Data {
	rank := 0
	var err error
	if movie.RankNo != "" {
		rank, err = strconv.Atoi(movie.RankNo)
		if err != nil {
			log.Println(err)
		}
	}
	properties := &model.Properties{
		Movie:    *genMovie(),
		Director: *genDirector(),
		Actor:    *genActor(),
		UpDate:   *genUpdate(),
		Region:   *genRegion(),
		Writer:   *genWriter(),
		Genre:    *genGenre(),
		Cover:    *genCover(),
		Duration: model.Duration{Number: movie.Duration},
		Language: *genLanguage(),
		Year:     model.Year{Number: int(movie.Year)},
		Imdb:     *genImdb(),
		Score:    model.Score{Number: movie.Score},
		Rank:     model.Rank{Number: int64(rank)},
	}

	body := &model.Data{
		Properties: *properties,
		Parent:     model.Parent{Type: "database_id", DatabaseID: config.Conf.DatabaseID},
	}
	return body
}

func genMovie() (mm *model.Movie) {
	title := model.Title{
		Type: "text",
		Text: struct {
			Content string `json:"content"`
		}(struct{ Content string }{Content: movie.Name}),
	}
	mm = &model.Movie{}
	mm.Title = append(mm.Title, title)
	return mm
}

func genDirector() (director *model.Director) {
	director = &model.Director{}
	for _, item := range movie.Director {
		if item != " " && item != "" {
			dr := model.Multi{Name: item}
			director.MultiSelect = append(director.MultiSelect, dr)
		}
	}
	return
}

func genActor() (actor *model.Actor) {
	actor = &model.Actor{}
	for _, item := range movie.Actor {
		if item != " " && item != "" {
			ac := model.Multi{Name: item}
			actor.MultiSelect = append(actor.MultiSelect, ac)
		}
	}
	return
}

func genUpdate() (update *model.UpDate) {
	update = &model.UpDate{}
	text := model.Text{Content: movie.ReleaseTime}
	rich := model.Rich{Type: "text", Text: text}
	update.RichText = append(update.RichText, rich)
	return
}

func genGenre() (genre *model.Genre) {
	genre = &model.Genre{}
	for _, item := range movie.Genre {
		if item != " " && item != "" {
			ge := model.Multi{Name: item}
			genre.MultiSelect = append(genre.MultiSelect, ge)
		}
	}
	if genre.MultiSelect == nil {
		em := model.Multi{Name: "None"}
		genre.MultiSelect = append(genre.MultiSelect, em)
	}
	return
}

func genRegion() (region *model.Region) {
	region = &model.Region{}
	for _, item := range movie.Region {
		if item != " " && item != "" {
			rg := model.Multi{Name: item}
			region.MultiSelect = append(region.MultiSelect, rg)
		}
	}
	return
}

func genWriter() (writer *model.Writer) {
	writer = &model.Writer{}
	for _, item := range movie.Writer {
		if item != " " && item != "" {
			wt := model.Multi{Name: item}
			writer.MultiSelect = append(writer.MultiSelect, wt)
		}
	}
	return
}

func genCover() (cover *model.Cover) {
	cover = &model.Cover{}
	url := model.External{Url: movie.Src}
	file := model.File{Name: "cover", Type: "external", External: url}
	cover.Files = append(cover.Files, file)
	return
}

func genLanguage() (language *model.Language) {
	language = &model.Language{}
	for _, item := range movie.Language {
		if item != " " && item != "" {
			lang := model.Multi{Name: item}
			language.MultiSelect = append(language.MultiSelect, lang)
		}
	}
	return
}

func genImdb() (imdb *model.Imdb) {
	imdb = &model.Imdb{}
	text := model.Text{Content: movie.Imdb}
	rich := model.Rich{Type: "text", Text: text}
	imdb.RichText = append(imdb.RichText, rich)
	return
}
