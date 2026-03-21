<template>
  <header class="fixed top-0 left-0 right-0 z-50 border-b border-one-dark-border bg-one-dark-bg-dark/80 backdrop-blur-xl">
    <div class="max-w-7xl mx-auto px-6 lg:px-8">
      <div class="flex items-center justify-between h-16">
        <div class="flex items-center gap-3">
          <div class="w-8 h-8 bg-gradient-to-br from-one-dark-accent via-one-dark-cyan to-one-dark-purple rounded-md flex items-center justify-center shadow-md shadow-one-dark-accent/20">
            <span class="text-one-dark-bg-darker font-bold text-lg">i</span>
          </div>
          <span class="text-xl font-bold text-one-dark-fg-light">iTools</span>
        </div>
        
        <!-- Desktop Navigation -->
        <nav class="hidden md:flex items-center gap-2">
          <router-link 
            to="/"
            :class="[
              'px-4 py-2 rounded-md text-sm font-medium transition-all duration-300',
              route.path === '/' 
                ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
                : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
            ]"
          >
            {{ getLocalizedText('home') }}
          </router-link>
          <router-link 
            to="/designer"
            :class="[
              'px-4 py-2 rounded-md text-sm font-medium transition-all duration-300',
              route.path === '/designer' 
                ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
                : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
            ]"
          >
            {{ getLocalizedText('designer') }}
          </router-link>
          <router-link 
            to="/download"
            :class="[
              'px-4 py-2 rounded-md text-sm font-medium transition-all duration-300',
              route.path === '/download' 
                ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
                : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
            ]"
          >
            {{ getLocalizedText('download') }}
          </router-link>
          <router-link 
            to="/document"
            :class="[
              'px-4 py-2 rounded-md text-sm font-medium transition-all duration-300',
              route.path === '/document' 
                ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
                : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
            ]"
          >
            {{ getLocalizedText('document') }}
          </router-link>
        </nav>

        <!-- Mobile Menu Button -->
        <div class="md:hidden flex items-center gap-4">
          <button 
            @click="toggleMobileMenu"
            class="p-2 rounded-md text-one-dark-fg hover:bg-one-dark-bg hover:border border-one-dark-border/50 transition-all duration-300"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path v-if="!showMobileMenu" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16"></path>
              <path v-else stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
          </button>
        </div>

        <!-- Language Selector (Desktop) -->
        <div class="hidden md:flex items-center gap-4">
            <div class="relative">
              <button class="flex items-center gap-2 px-3 py-2 rounded-md text-sm font-medium text-one-dark-fg hover:bg-one-dark-bg hover:border border-one-dark-border/50 transition-all duration-300" @click="toggleLanguageMenu">
                {{ currentLanguageName }}
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
                </svg>
              </button>
              <div v-if="showLanguageMenu" class="absolute right-0 mt-2 w-40 bg-one-dark-bg border border-one-dark-border rounded-md shadow-md py-2 z-50">
                <button 
                  v-for="lang in languages" 
                  :key="lang.code"
                  @click="switchLanguage(lang.code)"
                  class="block w-full text-left px-4 py-2 text-sm text-one-dark-fg hover:bg-one-dark-bg-light hover:text-one-dark-fg-light transition-colors"
                >
                  {{ lang.name }}
                </button>
              </div>
            </div>
          </div>
      </div>
      
      <!-- Mobile Navigation Menu -->
      <div v-if="showMobileMenu" class="md:hidden py-4 space-y-2">
        <router-link 
          to="/"
          @click="toggleMobileMenu"
          :class="[
            'block px-4 py-3 rounded-md text-sm font-medium transition-all duration-300',
            route.path === '/' 
              ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
              : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
          ]"
        >
          {{ getLocalizedText('home') }}
        </router-link>
        <router-link 
          to="/designer"
          @click="toggleMobileMenu"
          :class="[
            'block px-4 py-3 rounded-md text-sm font-medium transition-all duration-300',
            route.path === '/designer' 
              ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
              : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
          ]"
        >
          {{ getLocalizedText('designer') }}
        </router-link>
        <router-link 
          to="/download"
          @click="toggleMobileMenu"
          :class="[
            'block px-4 py-3 rounded-md text-sm font-medium transition-all duration-300',
            route.path === '/download' 
              ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
              : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
          ]"
        >
          {{ getLocalizedText('download') }}
        </router-link>
        <router-link 
          to="/document"
          @click="toggleMobileMenu"
          :class="[
            'block px-4 py-3 rounded-md text-sm font-medium transition-all duration-300',
            route.path === '/document' 
              ? 'bg-one-dark-bg border border-one-dark-accent text-one-dark-accent' 
              : 'text-one-dark-fg hover:text-one-dark-fg-light hover:bg-one-dark-bg hover:border border-one-dark-border/50'
          ]"
        >
          {{ getLocalizedText('document') }}
        </router-link>
        
        <!-- Mobile Language Selector -->
        <div class="relative">
          <button 
            class="flex items-center justify-between w-full px-4 py-3 rounded-md text-sm font-medium text-one-dark-fg hover:bg-one-dark-bg hover:border border-one-dark-border/50 transition-all duration-300"
            @click="toggleMobileLanguageMenu"
          >
            <span>{{ currentLanguageName }}</span>
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path>
            </svg>
          </button>
          <div v-if="showMobileLanguageMenu" class="mt-2 w-full bg-one-dark-bg border border-one-dark-border rounded-md shadow-md py-2 z-50">
            <button 
              v-for="lang in languages" 
              :key="lang.code"
              @click="switchLanguage(lang.code); toggleMobileLanguageMenu()"
              class="block w-full text-left px-4 py-2 text-sm text-one-dark-fg hover:bg-one-dark-bg-light hover:text-one-dark-fg-light transition-colors"
            >
              {{ lang.name }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </header>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from "vue";
import { useRoute } from "vue-router";

const route = useRoute();
const showLanguageMenu = ref(false);
const showMobileMenu = ref(false);
const showMobileLanguageMenu = ref(false);
const currentLanguage = ref(localStorage.getItem("language") || "zh-hans");

const languages = [
	{ code: "zh-hans", name: "简体中文" },
	{ code: "en", name: "English" },
];

const currentLanguageName = computed(() => {
	const lang = languages.find((l) => l.code === currentLanguage.value);
	return lang ? lang.name : "简体中文";
});

interface TranslationKeys {
	home: string;
	designer: string;
	download: string;
	document: string;
	[key: string]: string;
}

interface Translations {
	"zh-hans": TranslationKeys;
	en: TranslationKeys;
	[key: string]: TranslationKeys;
}

const translations: Translations = {
	"zh-hans": {
		home: "首页",
		designer: "设计器",
		download: "下载",
		document: "文档",
	},
	en: {
		home: "Home",
		designer: "Designer",
		download: "Download",
		document: "Document",
	},
};

function getLocalizedText(key: string): string {
	return (
		translations[currentLanguage.value]?.[key] ||
		translations["zh-hans"][key] ||
		key
	);
}

function toggleLanguageMenu() {
	showLanguageMenu.value = !showLanguageMenu.value;
}

function toggleMobileMenu() {
	showMobileMenu.value = !showMobileMenu.value;
}

function toggleMobileLanguageMenu() {
	showMobileLanguageMenu.value = !showMobileLanguageMenu.value;
}

function switchLanguage(code: string) {
	currentLanguage.value = code;
	localStorage.setItem("language", code);
	showLanguageMenu.value = false;
	showMobileLanguageMenu.value = false;
	// 重新加载当前页面以应用语言变化
	window.location.reload();
}

function handleClickOutside(event: MouseEvent) {
	const target = event.target as HTMLElement;
	if (!target.closest(".relative")) {
		showLanguageMenu.value = false;
	}
	if (!target.closest("header")) {
		showMobileMenu.value = false;
		showMobileLanguageMenu.value = false;
	}
}

onMounted(() => {
	document.addEventListener("click", handleClickOutside);
});

onUnmounted(() => {
	document.removeEventListener("click", handleClickOutside);
});
</script>