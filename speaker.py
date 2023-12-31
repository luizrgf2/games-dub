import gtts;
import sounddevice as sd
import soundfile as sf
import os
import pyttsx3


sd.default.latency = "low"

class App :

    def __init__(self) -> None:
        self.text_caption1 = ""
        self.text_caption2 = ""
        self.text_caption3 = ""

        self.current_caption = 1

    def read_text_from_cache(self):
        path_caption1 = os.path.join("caption1.txt")
        path_caption2 = os.path.join("caption2.txt")
        path_caption3 = os.path.join("caption3.txt")

        text_caption1 = ""
        text_caption2 = ""
        text_caption3 = ""


        if os.path.exists(path_caption1): 
            text_caption1 = open(path_caption1,"r",encoding="utf8").read()

        if os.path.exists(path_caption2): 
            text_caption2 = open(path_caption2,"r",encoding="utf8").read()
            
        if os.path.exists(path_caption3): 
            text_caption3 = open(path_caption3,"r",encoding="utf8").read()

        if not text_caption1 == self.text_caption1:
            self.text_caption1 = text_caption1
            return text_caption1
        
        elif not text_caption2 == self.text_caption2:
            self.text_caption2 = text_caption2
            return text_caption2
        
        elif not text_caption3 == self.text_caption3:
            self.text_caption3 = text_caption3
            return text_caption3

        return ""

    def select_voice(self):
        while(True):
            select = input("1: voice gtts\n2: voice pyttsx3")
            if(select.strip() == "1"):
                return self.speak
            elif(select.strip() == "2"):
                return self.speakAnotherTTS
            else:
                print("Escolha uma voz!")
                continue

    def speak(self):
        if len(self.text_to_speak) > 0:
            caption = gtts.gTTS(self.text_to_speak.replace("...",""),lang="pt-br")
            caption.save("caption.mp3")
            audio_data, sample_rate = sf.read("caption.mp3")
            sd.play(audio_data,33000)
            sd.wait()
    
    def speakAnotherTTS(self):
        if len(self.text_to_speak) > 0:
            engine = pyttsx3.init()
            voices = engine.getProperty('voices') 
            engine.setProperty("voice", "brazil")
            engine.save_to_file(self.text_to_speak,"caption.mp3")
            engine.runAndWait()
            audio_data, sample_rate = sf.read("caption.mp3")
            sd.play(audio_data,28000)
            sd.wait()

    def run(self):

        speaker = self.select_voice()

        while True:
            text_from_cache = self.read_text_from_cache()

            if not text_from_cache == "":
                print(text_from_cache)
                self.text_to_speak = text_from_cache
                speaker()

if __name__ == "__main__":
    app = App()
    app.run()