package posts

import (
	"fmt"
	"github.com/charmbracelet/lipgloss"
	"github.com/charmbracelet/lipgloss/table"
	"github.com/danielsoro/wordpress-cli/lib/wordpress/client"
	"github.com/spf13/cobra"
	"log/slog"
	"strconv"
)

type ListCommand struct {
	Command *cobra.Command
}

func NewListCommand(clientType client.WordPressClientType) ListCommand {
	return ListCommand{
		Command: &cobra.Command{
			Use:   "list",
			Short: "List posts from wordpress",
			Run: func(cmd *cobra.Command, args []string) {
				wordPressClient, err := client.NewWordPressClient(clientType)
				if err != nil {
					slog.Error(err.Error())
					return
				}

				posts := wordPressClient.GetPosts()

				if len(posts) == 0 {
					fmt.Println("No posts find")
					return
				}

				var rows [][]string

				t := table.New().
					Border(lipgloss.NormalBorder()).
					BorderStyle(lipgloss.NewStyle().Foreground(lipgloss.Color("99"))).
					Headers("ID", "Title", "Content", "Link", "Date")

				for _, p := range posts {
					rows = append(rows, []string{
						strconv.Itoa(p.ID), p.Title.Rendered, p.Content.Rendered, p.Link, p.Date,
					})
				}

				t.Rows(rows...)
				fmt.Println(t)
			},
		},
	}
}
