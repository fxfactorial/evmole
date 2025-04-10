package evmole

/*
   #cgo LDFLAGS: -L${SRCDIR}/../target/debug/ ${SRCDIR}/../target/debug/libevmole.a  -ldl
#include "./evmole.h"
#include <stdlib.h>
*/
import "C"
import (
	"encoding/json"
	"fmt"
	"unsafe"
)

type Row struct {
	FunctionSelector string
	Parameters       string
	Mutability       string
}

func Selectors(contract string) ([]Row, error) {
	str2 := C.CString(contract)
	defer C.free(unsafe.Pointer(str2))
	givenBack := C.produce_selectors(str2)
	defer C.free(unsafe.Pointer(givenBack))
	copied := C.GoString(givenBack)

	var (
		_selectors [][]string
		selectors  []Row
	)

	if err := json.Unmarshal([]byte(copied), &_selectors); err != nil {
		return nil, err
	}

	for _, k := range _selectors {
		if len(k) != 3 {
			return nil, fmt.Errorf("incorrect length")
		}
		selectors = append(selectors, Row{
			FunctionSelector: k[0],
			Parameters:       k[1],
			Mutability:       k[2],
		})

	}

	return selectors, nil
}
