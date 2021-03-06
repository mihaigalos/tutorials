package main

import "testing"

func TestGenesis_whenTypical(t *testing.T) {
	expected := true

	entries := []BlockchainEntry{}
	entries = append(entries,
		BlockchainEntry{
			"0000000000000000000000000000000000000000000000000000000000000000",
			"Genesis Block",
			"08fda3bd700bcbdf109890811a97d4ac98d0600799bcc53e57226bfbd8c5d56e",
		})

	chain := Blockchain{entries, uint64(len(entries))}
	actual := chain.validate()

	if actual != expected {
		t.Errorf("No Match: %d != %d", actual, expected)
	}
}

func TestTwoElements_whenTypical(t *testing.T) {
	expected := true

	entries := []BlockchainEntry{}
	entries = append(entries,
		BlockchainEntry{
			"0000000000000000000000000000000000000000000000000000000000000000",
			"Genesis Block",
			"08fda3bd700bcbdf109890811a97d4ac98d0600799bcc53e57226bfbd8c5d56e",
		})

	entries = append(entries,
		BlockchainEntry{
			"08fda3bd700bcbdf109890811a97d4ac98d0600799bcc53e57226bfbd8c5d56e",
			"FooBarEntry",
			"05810972d278e98049f70f1f7916c9b3c99ef1baf7afd05fa9929439257ecd51",
		})

	chain := Blockchain{entries, uint64(len(entries))}
	actual := chain.validate()

	if actual != expected {
		t.Errorf("No Match: %d != %d", actual, expected)
	}
}

func TestThreeElements_whenTypical(t *testing.T) {
	expected := true

	entries := []BlockchainEntry{}
	entries = append(entries,
		BlockchainEntry{
			"0000000000000000000000000000000000000000000000000000000000000000",
			"Genesis Block",
			"08fda3bd700bcbdf109890811a97d4ac98d0600799bcc53e57226bfbd8c5d56e",
		})

	entries = append(entries,
		BlockchainEntry{
			"08fda3bd700bcbdf109890811a97d4ac98d0600799bcc53e57226bfbd8c5d56e",
			"FooBarEntry",
			"05810972d278e98049f70f1f7916c9b3c99ef1baf7afd05fa9929439257ecd51",
		})

	entries = append(entries,
		BlockchainEntry{
			"05810972d278e98049f70f1f7916c9b3c99ef1baf7afd05fa9929439257ecd51",
			"FooBarEntry",
			"656e445334d79b8ece05d494a884d69ec2b422b4bbc39a111b799a35e2eaf009",
		})

	chain := Blockchain{entries, uint64(len(entries))}
	actual := chain.validate()

	if actual != expected {
		t.Errorf("No Match: %d != %d", actual, expected)
	}
}

func TestAddThreeElements_whenTypical(t *testing.T) {
	expected := true

	entries := []BlockchainEntry{}
	entries = append(entries,
		BlockchainEntry{
			"0000000000000000000000000000000000000000000000000000000000000000",
			"Genesis Block",
			"08fda3bd700bcbdf109890811a97d4ac98d0600799bcc53e57226bfbd8c5d56e",
		})
	chain := Blockchain{entries, uint64(len(entries))}
	chain.add("FooBarEntry")
	chain.add("FooBarEntry")

	actual := chain.validate()

	if actual != expected {
		t.Errorf("No Match: %d != %d", actual, expected)
	}
}
func TestAddThreeElementsFails_whenInvalidChecksumTypical(t *testing.T) {
	expected := false

	entries := []BlockchainEntry{}
	entries = append(entries,
		BlockchainEntry{
			"0000000000000000000000000000000000000000000000000000000000000000",
			"Genesis Block",
			"AAAAa3bd700bcbdf109890811a97d4ac98d0600799bcc53e57226bfbd8c5d56e",
		})
	chain := Blockchain{entries, uint64(len(entries))}
	chain.add("FooBarEntry")
	chain.add("FooBarEntry")

	actual := chain.validate()

	if actual != expected {
		t.Errorf("No Match: %d != %d", actual, expected)
	}
}
