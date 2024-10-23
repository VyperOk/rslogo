FOLDER="logo_examples"

for file in "$FOLDER"/*; do
	base_name="${file:r}"

	output=$(echo "$file" | sed 's/\.lg$/.svg/;s/logo_examples/logo_examples_out/')
	if [ -f "$file" ]; then
		echo "Running caro run for $file"
		6991 rslogo -- "$file" "$output" 200 200
		if [ $? -ne 0 ]; then
			echo "Error" > "$output"
		fi
	fi
done
