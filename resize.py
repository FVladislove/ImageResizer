import os
import logging
import time

from PIL import Image
from PIL.Image import Resampling


def initialize_logging():
    logging.basicConfig(
        filename='image_processing.log',  # Log file path
        level=logging.ERROR,  # Log level (only log errors and above)
        format='%(asctime)s - %(levelname)s - %(message)s',  # Log format
        filemode='a'  # Append to the log file (use 'w' to overwrite)
    )


def convert_images(folder_path: str, level=1):
    print("".join(["\t" * level, "L-- ", folder_path]))

    for filename in os.listdir(folder_path):
        item_path = os.path.join(folder_path, filename)
        if os.path.isdir(item_path):
            convert_images(item_path, level + 1)

        if filename.endswith((".jpg", ".png")):
            try:
                image = Image.open(item_path)
            except FileNotFoundError as e:
                initialize_logging()
                logging.error(f"File not found: {item_path}  {e}")
                continue

            print("".join(["\t" * level, "\tL-- ", filename]))

            width, height = image.size

            # through trials, it was determined that, while maintaining the ratio,
            # it is possible to set the smallest of the values of ~2000 pixels
            # less is possible, but in this way, the file is not too large
            # and the image quality remains at a high level
            if max(width, height) <= 2000:
                image.close()
                continue

            resize_coefficient = 2000 / max(width, height)

            image.resize((int(width * resize_coefficient), int(height * resize_coefficient)), resample=Resampling.LANCZOS).save(item_path)
            image.close()

try:
    script_path = os.path.abspath(__file__)
    script_dir = os.path.dirname(script_path)
    start = time.time_ns()
    convert_images(script_dir)
    end = time.time_ns()
    print((end - start) / 1000000)
# bad style, but I haven't time (and desire) to write tests
# and check all possible errors
except Exception as e:
    initialize_logging()
    logging.error(f"Exception raised  {e}")
