package config_test

import (
	"testing"

	"github.com/danielsoro/wordpress-cli/lib/config"
)

func TestConfig(t *testing.T) {
	config := config.NewConfigWithPath("../../test/resources/")

	if config.Credentials.Username != "foo" {
		t.Fatalf("Expecting %s, but was %s", "foo", config.Credentials.Username)
	}

	if config.Credentials.Password != "bar" {
		t.Fatalf("Expection %s, but was %s", "bar", config.Credentials.Password)
	}

	if config.Url != "https://foo.bar/wp-json/wp/v2" {
		t.Fatalf("Expecting %s, but was %s", "https://foo.bar/wp-json/wp/v2", config.Url)
	}
}
