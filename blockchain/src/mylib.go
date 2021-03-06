package main

import (
	"crypto/sha256"
	"fmt"
)

type BlockchainEntry struct {
	parentHash string
	content    string
	hash       string
}

type Blockchain struct {
	entries  []BlockchainEntry
	capacity uint64
}

func (entry *BlockchainEntry) convolute() string {
	return entry.content + entry.parentHash
}

func (entry *BlockchainEntry) validate(i uint64) bool {
	computedHash := fmt.Sprintf("%x", sha256.Sum256([]byte(entry.convolute())))

	if entry.hash != computedHash {
		fmt.Printf("[ERROR] Element %d :: Expected hash %s != Computed hash %s", i, entry.hash, computedHash)
		return false
	}
	return true
}

func (entry *BlockchainEntry) computeHash() [32]byte {
	return sha256.Sum256([]byte(entry.convolute()))
}

func (chain *Blockchain) validate() bool {
	for i := uint64(0); i < chain.capacity; i++ {
		if !chain.entries[i].validate(i) {
			return false
		}
	}
	return true
}

func (chain *Blockchain) add(content string) {
	entry := BlockchainEntry{chain.entries[len(chain.entries)-1].hash, content, "0000"}
	entry.hash = fmt.Sprintf("%x", sha256.Sum256([]byte(entry.convolute())))
	chain.entries = append(chain.entries, entry)
}
