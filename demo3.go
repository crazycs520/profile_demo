package main

import (
	"fmt"
	"sync/atomic"
	"time"
)

const SampleFreq = 1000
const ReportInterval = time.Second * 2

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
	taskCh := make(chan *Task, 128)
	// start the worker
	workerCount := 2
	for i := 0; i < workerCount; i++ {
		w := NewWorker()
		go w.Run(taskCh)
	}
	// send task to worker.
	for {
		for id := 1; id <= 4; id++ {
			task := &Task{
				tag:  int64(id),
				load: id,
			}
			taskCh <- task
			// time.Sleep(time.Millisecond * 100)
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

func (w *Worker) Run(taskCh chan *Task) {
	for {
		// get task from channel
		task := <-taskCh

		// set current running tag.
		atomic.StoreInt64(&w.tag, task.tag)

		task.Run()

		// clear the tag.
		atomic.StoreInt64(&w.tag, 0)
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
	sampleTicker := time.NewTicker(time.Second / time.Duration(SampleFreq))
	reportTicker := time.NewTicker(ReportInterval)
	defer func() {
		sampleTicker.Stop()
		reportTicker.Stop()
	}()
	summary := make(map[int64]int)
	tags := []*int64{}

	for {
		select {
		case <-sampleTicker.C:
			for _, tagPtr := range tags {
				tag := atomic.LoadInt64(tagPtr)
				summary[tag]++
			}
		case tag := <-r.tagRegister:
			tags = append(tags, tag)
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
			for _, v := range summary {
				total += v
			}
			fmt.Printf("\n\n%v: task profile:\n", time.Now().Format("2006-01-02T15:04:05"))
			for tag, v := range summary {
				usage := float64(v) / float64(total) * 100
				if tag != 0 {
					fmt.Printf("task_id: %v, cost: %.0f%% \n", tag, usage)
				} else {
					fmt.Printf("idle            : %.0f%% \n", usage)
				}
			}
		}
	}
}

type Task struct {
	tag  int64
	load int
}

const CYCLE = 50000000

func (t *Task) Run() {
	n := t.load * CYCLE
	sum := 0
	for i := 0; i < n; i++ {
		sum += i
	}
}
