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

i = 0
extension_calc = 0
image_opening = 0
image_resizing = 0
image_saving = 0
def convert_images(folder_path: str, level=1):
    global i
    global extension_calc
    global image_opening
    global image_resizing
    global image_saving

    print("".join(["\t" * level, "L-- ", folder_path]))

    for filename in os.listdir(folder_path):
        item_path = os.path.join(folder_path, filename)
        if os.path.isdir(item_path):
            convert_images(item_path, level + 1)

        if filename.endswith((".jpg", ".png", ".jpeg")):
            try:
                i += 1
                start = time.time_ns()
                image = Image.open(item_path)
                image_opening += time.time_ns() - start
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

            start = time.time_ns()
            resize_coefficient = 2000 / max(width, height)
            extension_calc += time.time_ns() - start

            start = time.time_ns()
            resized_image = image.resize(
                (int(width * resize_coefficient), int(height * resize_coefficient)),
                resample=Resampling.LANCZOS)
            image_resizing = time.time_ns() - start

            start = time.time_ns()
            resized_image.save(item_path)
            image_saving = time.time_ns() - start
            image.close()

try:
    script_path = os.path.abspath(__file__)
    script_dir = os.path.dirname(script_path)
    script_dir += "/test_folder"
    start = time.time_ns()
    convert_images(script_dir)
    end = time.time_ns()
    print(f"Total images processed:\t{i}")
    print(f"Image opening mean:\t{image_opening / 1e6}ms")
    print(f"Extension calc mean:\t{extension_calc / 1e6}ms")
    print(f"Image resizing mean:\t{image_resizing / 1e6}ms")
    print(f"Image saving mean:\t{image_saving / 1e6}ms")
    print(f"Total:\t{(end - start) / 1e6}ms")
# bad style, but I haven't time (and desire) to write tests
# and check all possible errors
except Exception as e:
    initialize_logging()
    logging.error(f"Exception raised  {e}")
