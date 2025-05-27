<template>
  <div class="titlebar" @mousedown="startDragging">
    <div class="titlebar-title">
      Laskutin - Jäsenmaksulaskutus
    </div>
    <div class="titlebar-controls">
      <button class="titlebar-button" @click="minimizeWindow" title="Pienennä">
        <span class="icon icon-minimize"></span>
      </button>
      <button class="titlebar-button" @click="maximizeWindow" title="Suurenna/Palauta">
        <span class="icon" :class="isMaximized ? 'icon-restore' : 'icon-maximize'"></span>
      </button>
      <button class="titlebar-button titlebar-close" @click="closeWindow" title="Sulje">
        <span class="icon icon-close"></span>
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { getCurrentWindow } from '@tauri-apps/api/window'

const appWindow = getCurrentWindow()
const isMaximized = ref(false)

const startDragging = async (event: MouseEvent) => {
  // Älä aloita raahaamista jos klikataan painikkeita tai ikkuna on maksimoitu
  if ((event.target as HTMLElement).closest('.titlebar-controls') || isMaximized.value) {
    return
  }
  try {
    await appWindow.startDragging()
  } catch (error) {
    console.error('Failed to start dragging:', error)
  }
}

const minimizeWindow = async () => {
  try {
    await appWindow.minimize()
  } catch (error) {
    console.error('Failed to minimize window:', error)
  }
}

const maximizeWindow = async () => {
  try {
    await appWindow.toggleMaximize()
    isMaximized.value = await appWindow.isMaximized()
  } catch (error) {
    console.error('Failed to maximize window:', error)
  }
}

const closeWindow = async () => {
  try {
    await appWindow.close()
  } catch (error) {
    console.error('Failed to close window:', error)
  }
}

// Kuuntele ikkunan tilaa
onMounted(async () => {
  try {
    isMaximized.value = await appWindow.isMaximized()
  } catch (error) {
    console.error('Failed to get initial maximize state:', error)
  }
})
</script>

<style scoped>
.titlebar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: 30px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  display: flex;
  align-items: center;
  justify-content: space-between;
  z-index: 1000;
  user-select: none;
}

.titlebar-title {
  flex: 1;
  text-align: center;
  color: white;
  font-size: 13px;
  font-weight: 500;
  pointer-events: none;
}

.titlebar-controls {
  display: flex;
}

.titlebar-button {
  width: 46px;
  height: 30px;
  border: none;
  background: transparent;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  transition: background-color 0.2s ease;
}

.titlebar-button:hover {
  background: rgba(255, 255, 255, 0.1);
}

.titlebar-button.titlebar-close:hover {
  background: #e81123;
}

.titlebar-button .icon {
  width: 14px;
  height: 14px;
  filter: brightness(0) invert(1);
}

.titlebar-button:focus {
  outline: none;
}
</style>