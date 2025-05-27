<template>
  <div>
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900">Yhdistyksen asetukset</h1>
      <p class="mt-2 text-sm text-gray-600">
        Hallinnoi yhdistyksen perustietoja
      </p>
    </div>

    <div class="form-card">
      <div class="px-6 py-6">
        <h3 class="text-lg leading-6 font-medium text-gray-900 mb-6">
          Yhdistyksen tiedot
        </h3>
        
        <form @submit.prevent="saveOrganization" class="space-y-6">
          <div class="grid grid-cols-1 gap-6 sm:grid-cols-2">
            <div class="sm:col-span-2">
              <label class="form-label">
                Yhdistyksen nimi *
              </label>
              <input
                v-model="organizationForm.nimi"
                type="text"
                required
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                Katuosoite *
              </label>
              <input
                v-model="organizationForm.katuosoite"
                type="text"
                required
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                Postinumero *
              </label>
              <input
                v-model="organizationForm.postinumero"
                type="text"
                required
                pattern="[0-9]{5}"
                data-field-name="postinumero"
                @invalid="handleInvalidInput"
                @input="(event) => (event.target as HTMLInputElement).setCustomValidity('')"
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                Postitoimipaikka *
              </label>
              <input
                v-model="organizationForm.postitoimipaikka"
                type="text"
                required
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                Puhelinnumero
              </label>
              <input
                v-model="organizationForm.puhelinnumero"
                type="tel"
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                Sähköposti
              </label>
              <input
                v-model="organizationForm.sahkoposti"
                type="email"
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                Y-tunnus
              </label>
              <input
                v-model="organizationForm.y_tunnus"
                type="text"
                pattern="[0-9]{7}-[0-9]"
                data-field-name="y_tunnus"
                @invalid="handleInvalidInput"
                @input="(event) => (event.target as HTMLInputElement).setCustomValidity('')"
                placeholder="1234567-8"
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                Pankkitili (IBAN)
              </label>
              <input
                v-model="organizationForm.pankkitili"
                type="text"
                pattern="FI[0-9]{16}|FI[0-9]{2}( [0-9]{4}){4} [0-9]{2}"
                data-field-name="pankkitili"
                @invalid="handleInvalidInput"
                @input="(event) => (event.target as HTMLInputElement).setCustomValidity('')"
                placeholder="FI1410093000123458 tai FI12 3456 7890 1234 56"
                class="form-input"
              />
            </div>
            
            <div>
              <label class="form-label">
                BIC-koodi
              </label>
              <input
                v-model="organizationForm.bic"
                type="text"
                pattern="[A-Z]{6}[A-Z0-9]{2}([A-Z0-9]{3})?"
                data-field-name="bic"
                @invalid="handleInvalidInput"
                @input="(event) => (event.target as HTMLInputElement).setCustomValidity('')"
                placeholder="OKOYFIHH"
                class="form-input"
              />
            </div>
          </div>
          
          <div class="pt-6 border-t border-gray-200">
            <div class="flex justify-end">
              <button
                type="submit"
                :disabled="saving"
                class="btn btn-primary"
                :class="{'opacity-50': saving}"
              >
                {{ saving ? 'Tallennetaan...' : 'Tallenna tiedot' }}
              </button>
            </div>
          </div>
        </form>
      </div>
    </div>

    <!-- Onnistumisviesti -->
    <div
      v-if="showSuccess"
      class="fixed bottom-4 right-4 bg-green-100 border border-green-400 text-green-700 px-4 py-3 rounded z-50"
    >
      <div class="flex">
        <div class="flex-shrink-0">
          <svg class="h-5 w-5 text-green-400" viewBox="0 0 20 20" fill="currentColor">
            <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
          </svg>
        </div>
        <div class="ml-3">
          <p class="text-sm font-medium">
            Yhdistyksen tiedot tallennettu onnistuneesti!
          </p>
        </div>
      </div>
    </div>

    <!-- Virhe dialogi -->
    <AlertDialog
      :show="showErrorDialog"
      title="Virhe"
      :message="errorMessage"
      type="error"
      icon="error"
      @close="showErrorDialog = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import AlertDialog from './AlertDialog.vue'

interface Organization {
  nimi: string
  katuosoite: string
  postinumero: string
  postitoimipaikka: string
  puhelinnumero: string
  sahkoposti: string
  y_tunnus: string
  pankkitili: string
  bic: string
}

const organizationForm = ref<Organization>({
  nimi: '',
  katuosoite: '',
  postinumero: '',
  postitoimipaikka: '',
  puhelinnumero: '',
  sahkoposti: '',
  y_tunnus: '',
  pankkitili: '',
  bic: '',
})

const saving = ref(false)
const showSuccess = ref(false)
const showErrorDialog = ref(false)
const errorMessage = ref('')

// Custom validation function
const handleInvalidInput = (event: Event) => {
  const input = event.target as HTMLInputElement
  const fieldName = input.getAttribute('data-field-name')
  
  if (fieldName && input.validity.patternMismatch) {
    const messages: Record<string, string> = {
      'postinumero': 'Postinumeron muoto on virheellinen. Syötä 5-numeroinen postinumero.',
      'y_tunnus': 'Y-tunnuksen muoto on virheellinen. Syötä Y-tunnus muodossa 1234567-8.',
      'pankkitili': 'Tilinumeron muoto on virheellinen. Syötä suomalainen IBAN-tilinumero muodossa FI1410093000123458 tai FI12 3456 7890 1234 56.',
      'bic': 'BIC-koodin muoto on virheellinen. Syötä BIC-koodi muodossa OKOYFIHH.'
    }
    
    input.setCustomValidity(messages[fieldName] || 'Kentän muoto on virheellinen.')
  } else {
    input.setCustomValidity('')
  }
}

const saveOrganization = async () => {
  saving.value = true
  
  try {
    await invoke('update_organization', { organization: organizationForm.value })
    
    showSuccess.value = true
    setTimeout(() => {
      showSuccess.value = false
    }, 3000)
    
  } catch (error) {
    console.error('Virhe tallentaessa yhdistyksen tietoja:', error)
    errorMessage.value = 'Virhe tallentaessa yhdistyksen tietoja'
    showErrorDialog.value = true
  } finally {
    saving.value = false
  }
}

const loadOrganization = async () => {
  try {
    const organization = await invoke('get_organization')
    
    if (organization) {
      organizationForm.value = {
        nimi: (organization as any).nimi || '',
        katuosoite: (organization as any).katuosoite || '',
        postinumero: (organization as any).postinumero || '',
        postitoimipaikka: (organization as any).postitoimipaikka || '',
        puhelinnumero: (organization as any).puhelinnumero || '',
        sahkoposti: (organization as any).sahkoposti || '',
        y_tunnus: (organization as any).y_tunnus || '',
        pankkitili: (organization as any).pankkitili || '',
        bic: (organization as any).bic || '',
      }
    } else {
      // Aseta oletusarvot jos yhdistystä ei löydy
      organizationForm.value = {
        nimi: 'Esimerkkiyhdistys ry',
        katuosoite: 'Yhdistyskatu 1',
        postinumero: '00100',
        postitoimipaikka: 'Helsinki',
        puhelinnumero: '09-12345678',
        sahkoposti: 'yhdistys@example.fi',
        y_tunnus: '1234567-8',
        pankkitili: 'FI12 3456 7890 1234 56',
        bic: 'OKOYFIHH',
      }
    }
  } catch (error) {
    console.error('Virhe ladatessa yhdistyksen tietoja:', error)
  }
}

onMounted(() => {
  loadOrganization()
})
</script>