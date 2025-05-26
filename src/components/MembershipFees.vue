<template>
  <div>
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-3xl font-bold text-gray-900">Jäsenmaksut</h1>
        <p class="mt-2 text-sm text-gray-700">
          Määrittele jäsenmaksujen hinnat vuosittain
        </p>
      </div>
      <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
        <button
          @click="openAddModal"
          type="button"
          class="btn btn-primary"
        >
          Lisää jäsenmaksu
        </button>
      </div>
    </div>

    <!-- Suodattimet -->
    <div class="form-card">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-3">
        <div>
          <label class="form-label">Vuosi</label>
          <select
            v-model="filterYear"
            class="form-select"
          >
            <option value="">Kaikki vuodet</option>
            <option v-for="year in years" :key="year" :value="year">{{ year }}</option>
          </select>
        </div>
        <div>
          <label class="form-label">Jäsentyyppi</label>
          <select
            v-model="filterType"
            class="form-select"
          >
            <option value="">Kaikki tyypit</option>
            <option value="varsinainen">Varsinainen</option>
            <option value="kannatus">Kannatus</option>
            <option value="kunnia">Kunnia</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Jäsenmaksutaulukko -->
    <div class="data-table">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Vuosi
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Jäsentyyppi
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Summa
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
          <tr v-for="fee in filteredFees" :key="fee.id">
            <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-900">
              {{ fee.vuosi }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                class="badge"
                :class="getMemberTypeClass(fee.jasentyyppi)"
              >
                {{ getMemberTypeLabel(fee.jasentyyppi) }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ formatCurrency(fee.summa) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
              {{ formatDate(fee.created_at) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button
                @click="editFee(fee)"
                class="btn btn-sm btn-outline mr-2"
              >
                Muokkaa
              </button>
              <button
                @click="deleteFee(fee)"
                class="btn btn-sm btn-danger"
              >
                Poista
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="filteredFees.length === 0" class="text-center py-8">
        <p class="text-gray-500">Ei jäsenmaksuja määritelty</p>
      </div>
    </div>

    <!-- Lisää/Muokkaa jäsenmaksu -modaali -->
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
            {{ editingFee ? 'Muokkaa jäsenmaksua' : 'Lisää uusi jäsenmaksu' }}
          </h3>
          
          <form @submit.prevent="saveFee" class="space-y-4">
            <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
              <div>
                <label class="form-label">Vuosi *</label>
                <select
                  v-model="feeForm.vuosi"
                  required
                  :disabled="!!editingFee"
                  class="form-select"
                  :class="{'opacity-50': !!editingFee}"
                >
                  <option v-for="year in years" :key="year" :value="year">{{ year }}</option>
                </select>
              </div>
              <div>
                <label class="form-label">Jäsentyyppi *</label>
                <select
                  v-model="feeForm.jasentyyppi"
                  required
                  :disabled="!!editingFee"
                  class="form-select"
                  :class="{'opacity-50': !!editingFee}"
                >
                  <option value="varsinainen">Varsinainen</option>
                  <option value="kannatus">Kannatus</option>
                  <option value="kunnia">Kunnia</option>
                </select>
              </div>
            </div>
            
            <div>
              <label class="form-label">Summa (€) *</label>
              <input
                v-model.number="feeForm.summa"
                type="number"
                step="0.01"
                min="0"
                required
                class="form-input"
              />
            </div>
            
            <div v-if="!editingFee" class="bg-blue-50 border border-blue-200 rounded-md p-4">
              <div class="flex">
                <div class="flex-shrink-0">
                  <svg class="h-5 w-5 text-blue-400" viewBox="0 0 20 20" fill="currentColor">
                    <path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd" />
                  </svg>
                </div>
                <div class="ml-3">
                  <h3 class="text-sm font-medium text-blue-800">
                    Huomio
                  </h3>
                  <div class="mt-2 text-sm text-blue-700">
                    <p>Voit määritellä saman vuoden jäsenmaksun vain kerran kullekin jäsentyypille.</p>
                  </div>
                </div>
              </div>
            </div>
            
            <div class="flex justify-end space-x-3 pt-4">
              <button
                type="button"
                @click="closeModal"
                class="btn btn-secondary"
              >
                Peruuta
              </button>
              <button
                type="submit"
                class="btn btn-primary"
              >
                {{ editingFee ? 'Tallenna' : 'Lisää' }}
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

interface MembershipFee {
  id: number
  vuosi: number
  jasentyyppi: string
  summa: number
  created_at: string
}

const fees = ref<MembershipFee[]>([])
const filterYear = ref('')
const filterType = ref('')
const showModal = ref(false)
const editingFee = ref<MembershipFee | null>(null)

const years = computed(() => {
  const currentYear = new Date().getFullYear()
  return Array.from({ length: 10 }, (_, i) => currentYear - 5 + i)
})

const feeForm = ref({
  vuosi: new Date().getFullYear(),
  jasentyyppi: 'varsinainen',
  summa: 0,
})

const filteredFees = computed(() => {
  return fees.value.filter(fee => {
    const matchesYear = !filterYear.value || fee.vuosi.toString() === filterYear.value
    const matchesType = !filterType.value || fee.jasentyyppi === filterType.value
    
    return matchesYear && matchesType
  }).sort((a, b) => {
    // Lajittele ensin vuoden mukaan (uusin ensin), sitten jäsentyypin mukaan
    if (a.vuosi !== b.vuosi) {
      return b.vuosi - a.vuosi
    }
    return a.jasentyyppi.localeCompare(b.jasentyyppi)
  })
})

const getMemberTypeClass = (type: string) => {
  switch (type) {
    case 'varsinainen':
      return 'bg-blue-100 text-blue-800'
    case 'kannatus':
      return 'bg-green-100 text-green-800'
    case 'kunnia':
      return 'bg-purple-100 text-purple-800'
    default:
      return 'bg-gray-100 text-gray-800'
  }
}

const getMemberTypeLabel = (type: string) => {
  switch (type) {
    case 'varsinainen':
      return 'Varsinainen'
    case 'kannatus':
      return 'Kannatus'
    case 'kunnia':
      return 'Kunnia'
    default:
      return type
  }
}

const formatCurrency = (amount: number) => {
  return new Intl.NumberFormat('fi-FI', {
    style: 'currency',
    currency: 'EUR'
  }).format(amount)
}

const formatDate = (dateString: string) => {
  const date = new Date(dateString)
  const day = String(date.getDate()).padStart(2, '0')
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const year = date.getFullYear()
  return `${day}.${month}.${year}`
}

const openAddModal = () => {
  editingFee.value = null
  feeForm.value = {
    vuosi: new Date().getFullYear(),
    jasentyyppi: 'varsinainen',
    summa: 0,
  }
  showModal.value = true
}

const editFee = (fee: MembershipFee) => {
  editingFee.value = fee
  feeForm.value = {
    vuosi: fee.vuosi,
    jasentyyppi: fee.jasentyyppi,
    summa: fee.summa,
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingFee.value = null
}

const saveFee = async () => {
  try {
    if (editingFee.value) {
      await invoke('update_membership_fee', { 
        id: editingFee.value.id, 
        fee: feeForm.value 
      })
    } else {
      await invoke('create_membership_fee', { fee: feeForm.value })
    }
    await loadFees()
    closeModal()
  } catch (error) {
    console.error('Virhe tallentaessa jäsenmaksua:', error)
    alert('Virhe tallentaessa jäsenmaksua')
  }
}

const deleteFee = async (fee: MembershipFee) => {
  if (confirm(`Haluatko varmasti poistaa ${fee.vuosi} ${getMemberTypeLabel(fee.jasentyyppi)} jäsenmaksun?`)) {
    try {
      // TODO: Lisää delete_membership_fee komento
      // await invoke('delete_membership_fee', { id: fee.id })
      await loadFees()
    } catch (error) {
      console.error('Virhe poistaessa jäsenmaksua:', error)
      alert('Virhe poistaessa jäsenmaksua')
    }
  }
}

const loadFees = async () => {
  try {
    // TODO: Hae jäsenmaksut backend:ista
    // fees.value = await invoke('get_membership_fees')
    
    // Väliaikainen testidata
    fees.value = [
      {
        id: 1,
        vuosi: 2024,
        jasentyyppi: 'varsinainen',
        summa: 50.00,
        created_at: '2024-01-01T00:00:00Z',
      },
      {
        id: 2,
        vuosi: 2024,
        jasentyyppi: 'kannatus',
        summa: 25.00,
        created_at: '2024-01-01T00:00:00Z',
      },
      {
        id: 3,
        vuosi: 2024,
        jasentyyppi: 'kunnia',
        summa: 0.00,
        created_at: '2024-01-01T00:00:00Z',
      },
      {
        id: 4,
        vuosi: 2025,
        jasentyyppi: 'varsinainen',
        summa: 55.00,
        created_at: '2024-11-01T00:00:00Z',
      },
    ]
  } catch (error) {
    console.error('Virhe ladatessa jäsenmaksuja:', error)
  }
}

onMounted(() => {
  loadFees()
})
</script>