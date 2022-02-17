package main

import (
	"os"
	"testing"
)

func TestNewDeck(t *testing.T) {
	d := newDeck()
	if len(d) != 16 {
		t.Errorf("Expected deck length of 16, but got %v", len(d))
	}
}

func TestNewDeckAceOfSpades(t *testing.T) {
	d := newDeck()

	if d[0] != "Ace of Spades" {
		t.Errorf("Expected first card of Ace of Spades, but got %v.", d[0])
	}
}

func TestNewDeckFourOfClubs(t *testing.T) {
	d := newDeck()

	if d[len(d)-1] != "Four of Clubs" {
		t.Errorf("Expected first card of Four of Clubs, but got %v.", d[0])
	}
}

func TestSaveLoad(t *testing.T) {
	os.Remove("_decktesting")

	d := newDeck()
	d.saveToFile("_decktesting")

	loadedDeck := newDeckFromFile("_decktesting")

	if len(loadedDeck) != 16 {
		t.Errorf("Expected 16 cards in deck, got %v.", len(loadedDeck))
	}
}
