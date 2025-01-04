package wordpress

import (
	"fmt"
	"github.com/sogko/go-wordpress"
	"github.com/spf13/viper"
	"log/slog"
)

type Wordpress interface {
	GetPosts() []wordpress.Post
	CreatePost(title, content string) (*wordpress.Post, error)
}

var _ Wordpress = (*WithViper)(nil)

type WithViper struct {
	client *wordpress.Client
}

func NewWordpressWithViper() WithViper {
	return WithViper{
		client: wordpress.NewClient(&wordpress.Options{
			BaseAPIURL: viper.GetString("url"),
			Username:   viper.GetString("credentials.username"),
			Password:   viper.GetString("credentials.password"),
		}),
	}
}

func (w WithViper) CreatePost(title, content string) (*wordpress.Post, error) {
	post, _, _, err := w.client.Posts().Create(&wordpress.Post{
		Title: wordpress.Title{
			Raw: title,
		},
		Content: wordpress.Content{
			Raw: content,
		},
	})

	if err != nil {
		return nil, err
	}

	return post, nil
}

func (w WithViper) GetPosts() []wordpress.Post {
	posts, _, _, err := w.client.Posts().List(nil)
	if err != nil {
		slog.Debug("Error getting posts", slog.String("error", err.Error()))
		fmt.Println("Error getting posts")
	}

	return posts
}
