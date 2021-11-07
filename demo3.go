package main

import (
	"fmt"
	"sync/atomic"
	"time"
)

const SampleFreq = 100
const ReportInterval = time.Second * 1

var (
	GlobalCollector *Collector
	GlobalReporter  *Reporter
)

func init() {
	GlobalCollector = NewCollector()
	go GlobalCollector.Run()

	GlobalReporter = NewReporter()
	go GlobalReporter.Run()
}

func main() {
	requestCh := make(chan *Request, 128)
	// start the worker
	workerCount := 4
	for i := 0; i < workerCount; i++ {
		w := NewWorker()
		go w.Run(requestCh)
	}
	// send request to worker.
	for {
		for id := 1; id <= 4; id++ {
			req := &Request{tag: int64(id)}
			requestCh <- req
			//time.Sleep(time.Millisecond * 100)
		}
	}
}

type Worker struct {
	tag int64
}

func NewWorker() *Worker {
	w := &Worker{tag: 0}
	GlobalCollector.RegisterWorker(&w.tag)
	return w
}

func (w *Worker) Run(requestCh chan *Request) {
	for {
		// get request from channel
		req := <-requestCh

		// set current running tag.
		atomic.StoreInt64(&w.tag, req.tag)

		w.handleRequest(req)

		// clear the tag.
		atomic.StoreInt64(&w.tag, 0)
	}
}

const CYCLE = 50000000

func (w *Worker) handleRequest(req *Request) {
	var sum int64
	n := req.tag * CYCLE
	for i := int64(0); i < n; i++ {
		sum += i
	}
}

type Collector struct {
	tagRegister chan *int64
}

func NewCollector() *Collector {
	return &Collector{tagRegister: make(chan *int64, 8)}
}

func (r *Collector) RegisterWorker(tagPtr *int64) {
	r.tagRegister <- tagPtr
}

func (r *Collector) Run() {
	reportTicker := time.NewTicker(ReportInterval)
	sampleTicker := time.NewTicker(time.Second / time.Duration(SampleFreq))
	defer func() {
		sampleTicker.Stop()
		reportTicker.Stop()
	}()
	summary := make(map[int64]int)
	tags := []*int64{}

	for {
		select {
		case tag := <-r.tagRegister:
			tags = append(tags, tag)
		case <-sampleTicker.C:
			for _, tagPtr := range tags {
				tag := atomic.LoadInt64(tagPtr)
				summary[tag]++
			}
		case <-reportTicker.C:
			GlobalReporter.CollectData(summary)
			// clear current time window data.
			summary = make(map[int64]int)
		}
	}
}

type Reporter struct {
	dataChan chan map[int64]int
}

func NewReporter() *Reporter {
	return &Reporter{
		dataChan: make(chan map[int64]int, 1),
	}
}

func (r *Reporter) CollectData(summary map[int64]int) {
	r.dataChan <- summary
}

func (r *Reporter) Run() {
	for {
		select {
		case summary := <-r.dataChan:
			total := 0
			idle := summary[0]
			for _, v := range summary {
				total += v
			}
			ratio := float64(total) / float64(SampleFreq)
			totalUsage := float64(total-idle) / float64(total) * 100 * ratio
			fmt.Printf("\n\n%v total usage: %.0f%%, total sample count: %v, frequency: %v:\n", time.Now().Format(time.RFC3339), totalUsage, total, SampleFreq)
			for tag, v := range summary {
				usage := float64(v) / float64(total) * 100
				if tag != 0 {
					fmt.Printf("request_id: %v, cpu usage: %.0f%% \n", tag, usage)
				} else {
					fmt.Printf("idle                    : %.0f%% \n", usage)
				}
			}
		}
	}
}

type Request struct {
	tag int64
}
