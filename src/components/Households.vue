<template>
  <div class="page-content">
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-3xl font-bold text-gray-900">Taloudet</h1>
        <p class="mt-2 text-sm text-gray-700">
          Hallinnoi kotitalouksia ja osoitetietoja
        </p>
      </div>
      <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
        <button
          @click="openAddModal"
          type="button"
          class="btn btn-primary"
        >
          Lisää talous
        </button>
      </div>
    </div>

    <!-- Haku ja suodattimet -->
    <div class="mt-8">
      <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
        <div>
          <label class="form-label">Hae talous</label>
          <input
            v-model="searchTerm"
            type="text"
            placeholder="Hae nimen tai osoitteen mukaan..."
            class="form-input"
          />
        </div>
      </div>
    </div>

    <!-- Talouksien taulukko -->
    <div class="data-table households-table">
      <table class="table">
        <thead>
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Talouden nimi
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Osoite
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Jäsenmäärä
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Luotu
            </th>
            <th class="relative px-6 py-3">
              <span class="sr-only">Toiminnot</span>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="household in filteredHouseholds" :key="household.id">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm font-medium text-gray-900">
                {{ household.talouden_nimi || `Talous ${household.id}` }}
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm text-gray-900">
                {{ household.address?.katuosoite }}
              </div>
              <div class="text-sm text-gray-500">
                {{ household.address?.postinumero }} {{ household.address?.postitoimipaikka }}
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ household.member_count || 0 }} jäsentä
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ formatDate(household.created_at) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button
                @click="editHousehold(household)"
                class="btn btn-sm btn-outline mr-2"
              >
                Muokkaa
              </button>
              <button
                @click="deleteHousehold(household)"
                class="btn btn-sm btn-danger"
              >
                Poista
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="filteredHouseholds.length === 0" class="text-center py-8">
        <p class="text-gray-500">Ei talouksia löytynyt</p>
      </div>
    </div>

    <!-- Lisää/Muokkaa talous -modaali -->
    <div
      v-if="showModal"
      class="modal-overlay"
      @click="closeModal"
    >
      <div
        class="modal-content"
        @click.stop
      >
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 mb-4">
            {{ editingHousehold ? 'Muokkaa taloutta' : 'Lisää uusi talous' }}
          </h3>
          
          <form @submit.prevent="saveHousehold" class="space-y-6">
            <!-- Validation error -->
            <div v-if="validationError" class="bg-red-50 border border-red-200 rounded-md p-3">
              <div class="flex">
                <div class="text-sm text-red-800">
                  {{ validationError }}
                </div>
              </div>
            </div>
            
            <!-- Talouden nimi -->
            <div>
              <label class="form-label">Talouden nimi</label>
              <input
                v-model="householdForm.talouden_nimi"
                type="text"
                class="form-input"
                placeholder="esim. Korhosen perhe"
              />
            </div>
            
            <!-- Osoitetiedot -->
            <div class="space-y-4">
              <h4 class="text-md font-medium text-gray-900">Osoitetiedot</h4>
              
              <div>
                <label class="form-label">Katuosoite *</label>
                <input
                  v-model="householdForm.katuosoite"
                  type="text"
                  class="form-input"
                  required
                />
              </div>
              
              <div class="grid grid-cols-2 gap-4">
                <div>
                  <label class="form-label">Postinumero *</label>
                  <input
                    v-model="householdForm.postinumero"
                    type="text"
                    class="form-input"
                    required
                  />
                </div>
                <div>
                  <label class="form-label">Postitoimipaikka *</label>
                  <input
                    v-model="householdForm.postitoimipaikka"
                    type="text"
                    class="form-input"
                    required
                  />
                </div>
              </div>
            </div>
            
            <div class="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                @click="closeModal"
                class="btn btn-outline"
              >
                Peruuta
              </button>
              <button
                type="submit"
                class="btn btn-primary"
              >
                {{ editingHousehold ? 'Tallenna muutokset' : 'Lisää talous' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Address {
  id: number
  katuosoite: string
  postinumero: string
  postitoimipaikka: string
  talous_id: number
  created_at: string
  updated_at: string
}

interface Household {
  id: number
  talouden_nimi: string | null
  laskutusosoite_sama: boolean
  laskutusosoite_id: number | null
  created_at: string
  updated_at: string
  address?: Address
  member_count?: number
}

// Reactive data
const households = ref<Household[]>([])
const searchTerm = ref('')
const showModal = ref(false)
const editingHousehold = ref<Household | null>(null)
const validationError = ref('')

// Form data
const householdForm = ref({
  talouden_nimi: '',
  katuosoite: '',
  postinumero: '',
  postitoimipaikka: '',
})

// Computed
const filteredHouseholds = computed(() => {
  return households.value.filter(household => {
    const searchLower = searchTerm.value.toLowerCase()
    if (!searchLower) return true
    
    const name = household.talouden_nimi?.toLowerCase() || ''
    const address = household.address?.katuosoite?.toLowerCase() || ''
    const city = household.address?.postitoimipaikka?.toLowerCase() || ''
    
    return name.includes(searchLower) || 
           address.includes(searchLower) || 
           city.includes(searchLower)
  })
})

// Methods
const formatDate = (dateString: string) => {
  return new Date(dateString).toLocaleDateString('fi-FI')
}

const openAddModal = () => {
  editingHousehold.value = null
  validationError.value = ''
  householdForm.value = {
    talouden_nimi: '',
    katuosoite: '',
    postinumero: '',
    postitoimipaikka: '',
  }
  showModal.value = true
}

const editHousehold = (household: Household) => {
  editingHousehold.value = household
  validationError.value = ''
  householdForm.value = {
    talouden_nimi: household.talouden_nimi || '',
    katuosoite: household.address?.katuosoite || '',
    postinumero: household.address?.postinumero || '',
    postitoimipaikka: household.address?.postitoimipaikka || '',
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingHousehold.value = null
  validationError.value = ''
}

const saveHousehold = async () => {
  // Clear previous validation error
  validationError.value = ''
  
  // Validate required fields
  if (!householdForm.value.katuosoite.trim()) {
    validationError.value = 'Katuosoite on pakollinen.'
    return
  }
  if (!householdForm.value.postinumero.trim()) {
    validationError.value = 'Postinumero on pakollinen.'
    return
  }
  if (!householdForm.value.postitoimipaikka.trim()) {
    validationError.value = 'Postitoimipaikka on pakollinen.'
    return
  }

  try {
    if (editingHousehold.value) {
      console.log('Update household:', editingHousehold.value.id, householdForm.value)
      await invoke('update_household_with_address', {
        id: editingHousehold.value.id,
        householdData: householdForm.value
      })
    } else {
      console.log('Create household:', householdForm.value)
      await invoke('create_household_with_address', { 
        householdData: householdForm.value 
      })
    }
    await loadHouseholds()
    closeModal()
  } catch (error) {
    console.error('Virhe tallentaessa taloutta:', error)
    validationError.value = `Virhe tallentaessa taloutta: ${(error as any)?.message || error}`
  }
}

const deleteHousehold = async (household: Household) => {
  if (confirm(`Haluatko varmasti poistaa talouden "${household.talouden_nimi || `Talous ${household.id}`}"? Tämä poistaa myös kaikki talouden jäsenet.`)) {
    try {
      console.log('Delete household:', household.id)
      await invoke('delete_household', { id: household.id })
      await loadHouseholds()
    } catch (error) {
      console.error('Virhe poistaessa taloutta:', error)
      alert('Virhe poistaessa taloutta')
    }
  }
}

const loadHouseholds = async () => {
  try {
    console.log('Loading households from backend...')
    const householdsData = await invoke('get_households_with_addresses')
    console.log('Received households data:', householdsData)
    
    // Convert backend data to frontend format
    households.value = (householdsData as any[]).map((item: any) => ({
      id: item.id,
      talouden_nimi: item.talouden_nimi,
      laskutusosoite_sama: true,
      laskutusosoite_id: null,
      created_at: item.created_at,
      updated_at: item.created_at,
      address: {
        id: item.id,
        katuosoite: item.address.katuosoite,
        postinumero: item.address.postinumero,
        postitoimipaikka: item.address.postitoimipaikka,
        talous_id: item.id,
        created_at: item.created_at,
        updated_at: item.created_at,
      },
      member_count: item.member_count,
    }))
    
    console.log('Processed households:', households.value)
  } catch (error) {
    console.error('Virhe ladatessa talouksia:', error)
    households.value = []
  }
}

onMounted(() => {
  loadHouseholds()
})
</script>