#
# Quick crappy script to grab the results I care about
#

import re
import os
import os.path
import json

base = "target/criterion"
iteration_results = "target/iteration.csv"

is_result = re.compile(".*[\\/]new")
archetype = re.compile(r".*iteration-archetypes-([0-9]+)[\\/]([a-zA-Z-]+)[\\/]([0-9]+)")
iteration = re.compile(r".*iteration-([0-9]+)[/\\]([a-zA-Z\-]+)[/\\]([0-9+])")

basedir = os.path.join("target", "results")

# type -> points per archetypes -> archetypes
archetype_results = {}
archetypes_results_dir = os.path.join(basedir, "iteration-archetypes")
archetypes_result_path = lambda name: os.path.join(archetypes_results_dir, name)

# type -> number of components -> dataset size
iteration_results = {}
iteration_results_dir = os.path.join(basedir, "iteration")
iteration_result_path = lambda name: os.path.join(iteration_results_dir, name)

os.makedirs(archetypes_results_dir, exist_ok=True)
os.makedirs(iteration_results_dir, exist_ok=True)

def get_estimates(file):
	return json.load(file)["Median"]

def for_archetypes(path):
	match = archetype.match(path)
	if match == None:
		return
	with open(path) as file:
		archetype_results.setdefault(
			match.group(2), {}
		).setdefault(int(match.group(1)), {})[int(match.group(3))] = get_estimates(file)

def for_iteration(path):
	match = iteration.match(path)
	if match == None:
		return
	with open(path) as file:
		iteration_results.setdefault(
			match.group(2), {}
		).setdefault(int(match.group(3)), {})[int(match.group(1))] = get_estimates(file)

def find_results():
	for directory, _, files in os.walk(base):
		if is_result.match(directory) == None:
			continue
		if not "estimates.json" in files:
			continue
		path = os.path.join(directory, "estimates.json")
		for_archetypes(path)
		for_iteration(path)

def print_results(results, path_function, col_label, row_label):
	benches = sorted(results.keys())
	headers_cols = sorted(results[benches[0]].keys())
	headers_rows = sorted(results[benches[0]][headers_cols[0]].keys())
	fetch = lambda a, b, c: results[a][b][c]["point_estimate"]

	for bench in benches:
		with open(path_function(bench + ".csv"), "w+") as file:
			file.write(",".join(["N"] + [str(row) for row in headers_rows]) + "\n")
			for col in headers_cols:
				file.write(",".join([str(col)] + [str(fetch(bench, col, row)) for row in headers_rows]) + "\n")

	for col in headers_cols:
		with open(path_function(row_label + "-" + str(col) + ".csv"), "w+") as file:
			file.write(",".join(["N"] + [str(row) for row in headers_rows]) + "\n")
			for bench in benches:
				file.write(",".join([str(bench)] + [str(fetch(bench, col, row)) for row in headers_rows]) + "\n")

	for row in headers_rows:
		with open(path_function(col_label + "-" + str(row) + ".csv"), "w+") as file:
			file.write(",".join(["N"] + [str(col) for col in headers_cols]) + "\n")
			for bench in benches:
				file.write(",".join([str(bench)] + [str(fetch(bench, col, row)) for col in headers_cols]) + "\n")

find_results()
print_results(archetype_results, archetypes_result_path, "by-archetype-entities", "by-entities-archetypes")
print_results(iteration_results, iteration_result_path, "by-components-entities", "by-entites-components")
