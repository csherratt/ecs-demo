#
# Quick crappy script to grab the reuslts I care about
#

import re
import os
import os.path
import json

base = "target/criterion"
iteration_results = "target/iteration.csv"

is_result = re.compile(".*[\\/]new")
archetype = re.compile(r".*iteration-archetypes-([0-9]+)[\\/]([a-zA-Z-]+)[\\/]([0-9]+)")

archetype_results = {}
def for_archetypes(path):
	match = archetype.match(path)
	if match == None:
		return
	with open(path) as file:
		content = json.load(file)
		archetype_results.setdefault(
			match.group(2), {}
		).setdefault(int(match.group(1)), {})[int(match.group(3))] = content["Mean"]["point_estimate"]

def find_results():
	for directory, _, files in os.walk(base):
		if is_result.match(directory) == None:
			continue
		if not "estimates.json" in files:
			continue
		path = os.path.join(directory, "estimates.json")
		for_archetypes(path)

find_results()
for x, data in archetype_results.items():
	with open(x + ".csv", "w+") as file:
		headers_cols = sorted(data.keys())
		headers_rows = sorted(data[headers_cols[0]].keys())

		file.write(",")
		for row in headers_rows:
			file.write("%d," % row)
		file.write("\n")

		for col in headers_cols:
			file.write("%d," % col)
			for row in headers_rows:
				file.write("%f," % data[col][row])
			file.write("\n")
