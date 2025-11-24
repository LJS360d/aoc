# -- Makefile for Go Backend Microservice Project --

# .PHONY tells make these are not real files, they're targets
.PHONY: all build clean test run version docker lint fmt vet tidy install

# Project Information
VERSION := $(shell git describe --tags --abbrev=0 2>/dev/null || echo "0.0.0")
COMMIT_SHA := $(shell git rev-parse --short HEAD)
BUILD_TIME := $(shell date '+%Y-%m-%d %H:%M:%S')
GOOS := $(shell go env GOOS)
GOARCH := $(shell go env GOARCH)

BIN_DIR := ./bin
SRC_DIR := ./cmd

GO := go
LINTER := golangci-lint
LINTER_CONFIG := .golangci.yml
APP_NAME := gomssk
PORT := 8080

# Build Info (set by ldflags)
BUILD_FLAGS := -ldflags "-X main.Version=$(VERSION) -X main.CommitSHA=$(COMMIT_SHA) -X main.BuildTime=$(BUILD_TIME)"

version:
	@echo "$(VERSION)"

build: fmt vet
	@echo "Building the Go binary..."
	@mkdir -p $(BIN_DIR)
	GOOS=$(GOOS) GOARCH=$(GOARCH) $(GO) build $(BUILD_FLAGS) -o $(BIN_DIR)/$(APP_NAME)

clean:
	@echo "Cleaning build artifacts..."
	@rm -rf $(BIN_DIR)

# Run the service locally, natively
run: build
	@echo "Running the service..."
	$(BIN_DIR)/yourapp

test:
	@echo "Running tests..."
	$(GO) test -v ./...

docker:
	@echo "Building Docker image..."
	docker build -t $(APP_NAME):$(VERSION) .

docker-run:
	@echo "Running Docker container..."
	docker run --rm -p $(PORT):$(PORT) $(APP_NAME):$(VERSION)

lint:
	@echo "Running linters..."
	$(LINTER) run --config $(LINTER_CONFIG)

fmt:
	@echo "Formatting the code..."
	$(GO) fmt ./...

vet:
	@echo "Running go vet..."
	$(GO) vet ./...

tidy:
	@echo "Tidying up dependencies..."
	$(GO) mod tidy

install:
	@echo "Installing dependencies..."
	$(GO) mod download
	$(GO) mod tidy

ci: lint fmt vet test build
