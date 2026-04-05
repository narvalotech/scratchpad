#!/usr/bin/env bash

# Enable 'nullglob' so that if *.jpeg doesn't exist, it expands to nothing
# instead of the literal string "*.jpeg"
shopt -s nullglob
shopt -s nocaseglob

echo "Starting document ingestion pipeline..."
echo "----------------------------------------"

for dir in */; do
    # Remove trailing slash
    folder_name="${dir%/}"

    # Collect all images into an array
    files=("$folder_name"/*.jpg "$folder_name"/*.jpeg)

    # Check if we actually found files
    if [ ${#files[@]} -gt 0 ]; then
        echo "🚀 Processing folder: [$folder_name] (${#files[@]} images)"

        output_pdf="${folder_name}.pdf"
        temp_pdf="${folder_name}_temp_unprocessed.pdf"

        # 1. Convert JPEGs to a single temporary PDF
        # Removed 2>/dev/null so we can see if img2pdf complains
        if ! img2pdf "${files[@]}" --output "$temp_pdf"; then
            echo "❌ Error: img2pdf failed on folder [$folder_name]"
            continue
        fi

        # 2. Run OCRmyPDF
        # Using --force-ocr to ensure it handles the freshly created PDF
        if ocrmypdf --rotate-pages \
                 --rotate-pages-threshold 5 \
                 --deskew \
                 -l fra+eng \
                 --jobs $(nproc) \
                 "$temp_pdf" \
                 "$output_pdf"; then
            echo "✅ Finished: $output_pdf"
            rm "$temp_pdf"
        else
            echo "❌ Error: ocrmypdf failed on folder [$folder_name]"
        fi

        echo "----------------------------------------"
    else
        echo "⏭️  Skipping [$folder_name]: No images found."
    fi
    # exit 0
done

# Turn off the shell options we set
shopt -u nullglob
shopt -u nocaseglob

echo "Pipeline complete!"
