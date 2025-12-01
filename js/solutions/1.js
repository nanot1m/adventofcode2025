// @ts-check

import { iterate } from "../modules/itertools.js"
import { mod } from "../modules/lib.js"
import { t } from "../modules/parser.js"

export const useExample = false

export const exampleInput = `\
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82`

const start = 50

/** @typedef {ReturnType<typeof parseInput>} InputType */

export const parseInput = t.arr(
	t.str().map((str) => ({
		dir: str[0],
		count: Number(str.slice(1)),
	})),
).parse

/**
 * @param {InputType} input
 */
export function part1(input) {
	const values = input.map((x) => (x.dir === "L" ? -x.count : x.count))
	return iterate(start, (x, idx) => x + values[idx])
		.take(input.length)
		.count((x) => mod(x, 100) === 0)
}

/**
 * @param {InputType} input
 */
export function part2(input) {
	let acc = start
	let zeros = 0
	for (const val of input) {
		const sign = val.dir === "L" ? -1 : 1
		for (let i = 0; i < val.count; i++) {
			acc += sign
			if (mod(acc, 100) === 0) zeros++
		}
	}
	return zeros
}
