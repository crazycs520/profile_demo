package main

import (
	"bytes"
	"context"
	"fmt"
	"runtime/pprof"
	"strconv"
	"time"

	"github.com/google/pprof/profile"
)

func main() {
	for n := 0; n < 4; n++ {
		go func() {
			for {
				for id := 1; id <= 4; id++ {
					handleRequest(id)
				}
			}
		}()
	}

	buf := bytes.NewBuffer(nil)
	for {
		fmt.Println("---------------------------")
		buf.Reset()
		err := pprof.StartCPUProfile(buf)
		if err != nil {
			continue
		}

		time.Sleep(time.Second * 2)

		pprof.StopCPUProfile()

		p, err := profile.ParseData(buf.Bytes())
		if err != nil {
			continue
		}
		reqMap, total := parseCPUProfileByLabels(p)
		for id, load := range reqMap {
			fmt.Printf("request_id %v, cpu usage: %v%%\n", id, load*100/total)
		}
	}
}

const CYCLE = 10000000

func handleRequest(id int) {
	ctx := pprof.WithLabels(context.Background(), pprof.Labels("id", strconv.Itoa(id)))
	pprof.SetGoroutineLabels(ctx)
	defer pprof.SetGoroutineLabels(context.Background())

	// if id == 4 {
	// 	time.Sleep(time.Millisecond * 5)
	// 	return
	// }

	n := id * CYCLE
	sum := 0
	for i := 0; i < n; i++ {
		sum += i * 2
	}
}

func parseCPUProfileByLabels(p *profile.Profile) (map[string]int64, int64) {
	reqMap := make(map[string]int64)
	total := int64(0)
	idx := len(p.SampleType) - 1
	for _, s := range p.Sample {
		ids, ok := s.Label["id"]
		if !ok || len(ids) == 0 {
			continue
		}
		for _, id := range ids {
			reqMap[id] += s.Value[idx]
		}
		total += s.Value[idx]
	}
	return reqMap, total
}
