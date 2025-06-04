<template>
  <div class="page-content">
    <div class="sm:flex sm:items-center">
      <div class="sm:flex-auto">
        <h1 class="text-3xl font-bold text-gray-900">Jäsenet</h1>
        <p class="mt-2 text-sm text-gray-700">
          Hallinnoi yhdistyksen jäsentietoja
        </p>
      </div>
      <div class="mt-4 sm:mt-0 sm:ml-16 sm:flex-none">
        <button
          @click="openAddModal"
          type="button"
          class="btn btn-primary"
        >
          Lisää jäsen
        </button>
      </div>
    </div>

    <!-- Hakukenttä ja suodattimet -->
    <div class="form-card">
      <div class="grid grid-cols-1 gap-4 sm:grid-cols-4">
        <div>
          <label class="form-label">Haku</label>
          <input
            v-model="searchTerm"
            type="text"
            placeholder="Etsi nimellä..."
            class="form-input"
          />
        </div>
        <div>
          <label class="form-label">Jäsentyyppi</label>
          <select
            v-model="filterType"
            class="form-select"
          >
            <option value="">Kaikki</option>
            <option value="Varsinainen">Varsinainen</option>
            <option value="Kannatus">Kannatus</option>
            <option value="Kunnia">Kunnia</option>
            <option value="Nuorisojasen">Nuorisojasen</option>
          </select>
        </div>
        <div>
          <label class="form-label">Tila</label>
          <select
            v-model="filterActive"
            class="form-select"
          >
            <option value="">Kaikki</option>
            <option value="true">Aktiivinen</option>
            <option value="false">Passiivinen</option>
          </select>
        </div>
      </div>
    </div>

    <!-- Jäsentaulukko -->
    <div class="data-table members-table">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Nimi
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Yhteystiedot
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Jäsentyyppi
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Liittymispäivä
            </th>
            <th class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
              Tila
            </th>
            <th class="relative px-6 py-3">
              <span class="sr-only">Toiminnot</span>
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="member in filteredMembers" :key="member.id">
            <td class="px-6 py-4 whitespace-nowrap">
              <div class="text-sm font-medium text-gray-900">
                {{ member.etunimi }} {{ member.sukunimi }}
              </div>
              <div v-if="member.syntymaaika" class="text-sm text-gray-500">
                Syntynyt: {{ formatDate(member.syntymaaika) }}
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <div v-if="member.sahkoposti" class="text-sm text-gray-900">
                {{ member.sahkoposti }}
              </div>
              <div v-if="member.puhelinnumero" class="text-sm text-gray-500">
                {{ member.puhelinnumero }}
              </div>
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                class="badge"
                :class="getMemberTypeClass(member.jasentyyppi)"
              >
                {{ getMemberTypeLabel(member.jasentyyppi) }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-900">
              {{ formatDate(member.liittymispaiva) }}
            </td>
            <td class="px-6 py-4 whitespace-nowrap">
              <span
                class="badge"
                :class="member.aktiivinen ? 'badge-success' : 'badge-danger'"
              >
                {{ member.aktiivinen ? 'Aktiivinen' : 'Passiivinen' }}
              </span>
            </td>
            <td class="px-6 py-4 whitespace-nowrap text-right text-sm font-medium">
              <button
                @click="editMember(member)"
                class="btn btn-sm btn-outline mr-2"
              >
                Muokkaa
              </button>
              <button
                @click="deleteMember(member)"
                class="btn btn-sm btn-danger"
              >
                Poista
              </button>
            </td>
          </tr>
        </tbody>
      </table>
      
      <div v-if="filteredMembers.length === 0" class="text-center py-8">
        <p class="text-gray-500">Ei jäseniä</p>
      </div>
    </div>

    <!-- Lisää/Muokkaa jäsen -modaali -->
    <div
      v-if="showModal"
      class="modal-overlay"
      @click="closeModal"
    >
      <div
        class="modal-content wide"
        @click.stop
      >
        <div class="mt-3">
          <h3 class="text-lg font-medium text-gray-900 mb-4">
            {{ editingMember ? 'Muokkaa jäsentä' : 'Lisää uusi jäsen' }}
          </h3>
          
          <form @submit.prevent="saveMember" class="space-y-6">
            <!-- Validation error -->
            <div v-if="validationError" class="bg-red-50 border border-red-200 rounded-md p-3">
              <div class="flex">
                <div class="text-sm text-red-800">
                  {{ validationError }}
                </div>
              </div>
            </div>
            
            <!-- Nimi -->
            <div class="grid grid-cols-2 gap-6">
              <div>
                <label class="form-label">Etunimi *</label>
                <input
                  v-model="memberForm.etunimi"
                  type="text"
                  class="form-input"
                />
              </div>
              <div>
                <label class="form-label">Sukunimi *</label>
                <input
                  v-model="memberForm.sukunimi"
                  type="text"
                  class="form-input"
                />
              </div>
            </div>
            
            <!-- Yhteystiedot -->
            <div class="grid grid-cols-2 gap-6">
              <div>
                <label class="form-label">Sähköposti</label>
                <input
                  v-model="memberForm.sahkoposti"
                  type="email"
                  class="form-input"
                />
              </div>
              <div>
                <label class="form-label">Puhelinnumero</label>
                <input
                  v-model="memberForm.puhelinnumero"
                  type="tel"
                  class="form-input"
                />
              </div>
            </div>
            
            <!-- Syntymäaika -->
            <div class="grid grid-cols-2 gap-6">
              <div>
                <label class="form-label">Syntymäaika *</label>
                <VueDatePicker
                  v-model="memberForm.syntymaaika"
                  locale="fi"
                  format="dd.MM.yyyy"
                  :enable-time-picker="false"
                  auto-apply
                  :clearable="false"
                  placeholder="Valitse päivämäärä"
                />
              </div>
              <div>
                <!-- Empty div to maintain grid layout -->
              </div>
            </div>
            
            <!-- Jäsenyystiedot -->
            <div class="grid grid-cols-3 gap-6">
              <div>
                <label class="form-label">Jäsentyyppi *</label>
                <select
                  v-model="memberForm.jasentyyppi"
                  required
                  class="form-select"
                >
                  <option value="Varsinainen">Varsinainen</option>
                  <option value="Kannatus">Kannatus</option>
                  <option value="Kunnia">Kunnia</option>
                  <option value="Nuorisojasen">Nuorisojasen</option>
                </select>
              </div>
              <div>
                <label class="form-label">Liittymispäivä *</label>
                <VueDatePicker
                  v-model="memberForm.liittymispaiva"
                  locale="fi"
                  format="dd.MM.yyyy"
                  :enable-time-picker="false"
                  auto-apply
                  :clearable="false"
                  placeholder="Valitse päivämäärä"
                />
              </div>
              <div class="flex items-center pt-6">
                <input
                  v-model="memberForm.aktiivinen"
                  id="aktiivinen"
                  type="checkbox"
                  class="h-5 w-5 text-blue-600 focus:ring-blue-500 border-gray-300 rounded"
                />
                <label for="aktiivinen" class="ml-2 block text-sm text-gray-900">
                  Aktiivinen jäsen
                </label>
              </div>
            </div>
            
            <!-- Osoite/Talous tiedot -->
            <div class="space-y-4">
              <div>
                <label class="form-label">Osoitetiedot *</label>
                <select
                  v-model="memberForm.osoitetyyppi"
                  class="form-select"
                >
                  <option value="oma">Oma osoite (yhden hengen talous)</option>
                  <option value="talous">Liity olemassa olevaan talouteen</option>
                  <option value="uusi">Luo uusi talous</option>
                </select>
              </div>
              
              <!-- Liity olemassa olevaan talouteen -->
              <div v-if="memberForm.osoitetyyppi === 'talous'">
                <label class="form-label">Valitse talous *</label>
                <select
                  v-model="memberForm.talous_id"
                  class="form-select"
                >
                  <option value="">Valitse talous...</option>
                  <option v-for="talous in taloudet" :key="talous.id" :value="talous.id">
                    {{ talous.talouden_nimi }} - {{ talous.osoite }}
                  </option>
                </select>
              </div>
              
              <!-- Luo uusi talous -->
              <div v-if="memberForm.osoitetyyppi === 'uusi'" class="space-y-4">
                <div>
                  <label class="form-label">Talouden nimi</label>
                  <input
                    v-model="memberForm.talouden_nimi"
                    type="text"
                    placeholder="esim. Korhosen perhe"
                    class="form-input"
                  />
                  <div class="text-xs text-gray-500 mt-1">
                    Jos jätät tyhjäksi, käytetään jäsenen nimeä
                  </div>
                </div>
              </div>
              
              <!-- Osoitetiedot (oma osoite tai uusi talous) -->
              <div v-if="memberForm.osoitetyyppi === 'oma' || memberForm.osoitetyyppi === 'uusi'" class="space-y-4">
                <div class="grid grid-cols-1 gap-4 sm:grid-cols-2">
                  <div class="sm:col-span-2">
                    <label class="form-label">Katuosoite *</label>
                    <input
                      v-model="memberForm.katuosoite"
                      type="text"
                      placeholder="esim. Kotikatu 123 A 5"
                      class="form-input"
                    />
                  </div>
                  <div>
                    <label class="form-label">Postinumero *</label>
                    <input
                      v-model="memberForm.postinumero"
                      type="text"
                      pattern="[0-9]{5}"
                      placeholder="00100"
                      class="form-input"
                    />
                  </div>
                  <div>
                    <label class="form-label">Postitoimipaikka *</label>
                    <input
                      v-model="memberForm.postitoimipaikka"
                      type="text"
                      placeholder="Helsinki"
                      class="form-input"
                    />
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
                {{ editingMember ? 'Tallenna' : 'Lisää jäsen' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Vahvistus dialogi -->
    <ConfirmDialog
      :show="showConfirmDialog"
      title="Vahvista poisto"
      :message="confirmMessage"
      type="danger"
      icon="danger"
      confirm-text="Poista"
      cancel-text="Peruuta"
      @confirm="confirmDeleteMember"
      @cancel="cancelDeleteMember"
    />

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
import { ref, computed, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import ConfirmDialog from './ConfirmDialog.vue'
import AlertDialog from './AlertDialog.vue'

interface Member {
  id: number
  etunimi: string
  sukunimi: string
  henkilotunnus?: string
  syntymaaika?: string
  puhelinnumero?: string
  sahkoposti?: string
  liittymispaiva: string
  jasentyyppi: string
  aktiivinen: boolean
  talouden_nimi?: string
  katuosoite?: string
  postinumero?: string
  postitoimipaikka?: string
}

// Helper function for date formatting
const formatDate = (dateInput: string | Date) => {
  const date = dateInput instanceof Date ? dateInput : new Date(dateInput)
  const day = String(date.getDate()).padStart(2, '0')
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const year = date.getFullYear()
  return `${day}.${month}.${year}`
}

const members = ref<Member[]>([])
const taloudet = ref<{id: number, talouden_nimi: string, osoite: string}[]>([])
const searchTerm = ref('')
const filterType = ref('')
const filterActive = ref('')
const showModal = ref(false)
const editingMember = ref<Member | null>(null)
const validationError = ref('')
const showConfirmDialog = ref(false)
const showErrorDialog = ref(false)
const confirmMessage = ref('')
const errorMessage = ref('')
const memberToDelete = ref<Member | null>(null)

const memberForm = ref({
  etunimi: '',
  sukunimi: '',
  syntymaaika: null as Date | null,
  puhelinnumero: '',
  sahkoposti: '',
  liittymispaiva: new Date(),
  jasentyyppi: 'Varsinainen',
  aktiivinen: true,
  // Osoite/talous tiedot
  osoitetyyppi: 'oma' as 'oma' | 'talous' | 'uusi', // oma=pelkka osoite, talous=liity olemassa olevaan, uusi=luo uusi talous
  talous_id: null as number | null,
  talouden_nimi: '',
  katuosoite: '',
  postinumero: '',
  postitoimipaikka: '',
})

const filteredMembers = computed(() => {
  return members.value.filter(member => {
    const matchesSearch = !searchTerm.value || 
      `${member.etunimi} ${member.sukunimi}`.toLowerCase().includes(searchTerm.value.toLowerCase())
    
    const matchesType = !filterType.value || 
      member.jasentyyppi.toLowerCase() === filterType.value.toLowerCase()
    
    const matchesActive = !filterActive.value || 
      member.aktiivinen.toString() === filterActive.value
    
    return matchesSearch && matchesType && matchesActive
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
    case 'nuorisojasen':
      return 'bg-yellow-100 text-yellow-800'
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
    case 'nuorisojasen':
      return 'Nuorisojasen'
    default:
      return type
  }
}

const openAddModal = () => {
  editingMember.value = null
  validationError.value = ''
  memberForm.value = {
    etunimi: '',
    sukunimi: '',
    syntymaaika: null,
    puhelinnumero: '',
    sahkoposti: '',
    liittymispaiva: new Date(),
    jasentyyppi: 'Varsinainen',
    aktiivinen: true,
    osoitetyyppi: 'oma',
    talous_id: null,
    talouden_nimi: '',
    katuosoite: '',
    postinumero: '',
    postitoimipaikka: '',
  }
  showModal.value = true
}

const editMember = (member: Member) => {
  editingMember.value = member
  validationError.value = ''
  
  // Convert member type to capitalized form expected by frontend
  let jasentyyppi = member.jasentyyppi
  if (jasentyyppi === 'varsinainen') jasentyyppi = 'Varsinainen'
  if (jasentyyppi === 'kannatus') jasentyyppi = 'Kannatus'  
  if (jasentyyppi === 'kunnia') jasentyyppi = 'Kunnia'
  if (jasentyyppi === 'nuorisojasen') jasentyyppi = 'Nuorisojasen'
  
  // Determine address type based on household data
  let osoitetyyppi = 'oma'
  let talous_id = null
  
  // If member has a household name that's not just their own name, they're in a shared household
  if (member.talouden_nimi && member.talouden_nimi !== `${member.etunimi} ${member.sukunimi}`) {
    osoitetyyppi = 'talous'
    // Find the household ID from taloudet list
    const talous = taloudet.value.find(t => t.talouden_nimi === member.talouden_nimi)
    if (talous) {
      talous_id = talous.id
    }
  }
  
  memberForm.value = {
    etunimi: member.etunimi,
    sukunimi: member.sukunimi,
    syntymaaika: member.syntymaaika ? new Date(member.syntymaaika) : null,
    puhelinnumero: member.puhelinnumero || '',
    sahkoposti: member.sahkoposti || '',
    liittymispaiva: new Date(member.liittymispaiva),
    jasentyyppi: jasentyyppi,
    aktiivinen: member.aktiivinen,
    // Fill address fields from member data
    osoitetyyppi: osoitetyyppi as 'oma' | 'talous' | 'uusi',
    talous_id: talous_id,
    talouden_nimi: member.talouden_nimi || '',
    katuosoite: member.katuosoite || '',
    postinumero: member.postinumero || '',
    postitoimipaikka: member.postitoimipaikka || '',
  }
  showModal.value = true
}

const closeModal = () => {
  showModal.value = false
  editingMember.value = null
}

const saveMember = async () => {
  console.log('saveMember called')
  console.log('memberForm.value:', memberForm.value)
  
  // Clear previous validation error
  validationError.value = ''
  
  // Validate required fields
  if (!memberForm.value.etunimi.trim()) {
    console.log('Etunimi validation failed')
    validationError.value = 'Etunimi on pakollinen.'
    return
  }
  if (!memberForm.value.sukunimi.trim()) {
    console.log('Sukunimi validation failed')
    validationError.value = 'Sukunimi on pakollinen.'
    return
  }
  if (!memberForm.value.syntymaaika) {
    console.log('Syntymäaika validation failed')
    validationError.value = 'Syntymäaika on pakollinen. Valitse päivämäärä.'
    return
  }
  if (!memberForm.value.liittymispaiva) {
    console.log('Liittymispäivä validation failed')
    validationError.value = 'Liittymispäivä on pakollinen. Valitse päivämäärä.'
    return
  }
  
  // Validate address/household info
  if (memberForm.value.osoitetyyppi === 'talous' && !memberForm.value.talous_id) {
    validationError.value = 'Valitse talous jota liittyä.'
    return
  }
  
  if ((memberForm.value.osoitetyyppi === 'oma' || memberForm.value.osoitetyyppi === 'uusi')) {
    if (!memberForm.value.katuosoite.trim()) {
      validationError.value = 'Katuosoite on pakollinen.'
      return
    }
    if (!memberForm.value.postinumero.trim()) {
      validationError.value = 'Postinumero on pakollinen.'
      return
    }
    if (!memberForm.value.postitoimipaikka.trim()) {
      validationError.value = 'Postitoimipaikka on pakollinen.'
      return
    }
  }
  
  console.log('All validations passed')

  try {
    console.log('Trying to invoke Tauri command...')
    console.log('invoke function:', invoke)
    console.log('typeof invoke:', typeof invoke)
    
    // Prepare member data for backend - convert dates to strings
    const memberDataForBackend = {
      ...memberForm.value,
      syntymaaika: memberForm.value.syntymaaika ? memberForm.value.syntymaaika.toISOString().split('T')[0] : null,
      liittymispaiva: memberForm.value.liittymispaiva.toISOString().split('T')[0],
      henkilotunnus: null, // Removed as requested
    }
    
    if (editingMember.value) {
      console.log('Calling update_member_with_address with id:', editingMember.value.id, 'memberData:', memberDataForBackend)
      await invoke('update_member_with_address', { 
        id: editingMember.value.id, 
        memberData: memberDataForBackend 
      })
    } else {
      console.log('Calling create_member_with_address with member data:', memberDataForBackend)
      await invoke('create_member_with_address', { memberData: memberDataForBackend })
    }
    await loadMembers()
    closeModal()
  } catch (error) {
    console.error('Virhe tallentaessa jäsentä:', error)
    console.error('Error details:', error)
    validationError.value = `Virhe tallentaessa jäsentä: ${(error as any)?.message || error}`
  }
}

const deleteMember = async (member: Member) => {
  console.log('deleteMember called with:', member)
  memberToDelete.value = member
  confirmMessage.value = `Haluatko varmasti poistaa jäsenen ${member.etunimi} ${member.sukunimi}?`
  showConfirmDialog.value = true
  console.log('showConfirmDialog set to:', showConfirmDialog.value)
}

const confirmDeleteMember = async () => {
  console.log('confirmDeleteMember called')
  if (!memberToDelete.value) {
    console.log('No memberToDelete, returning')
    return
  }
  
  console.log('Deleting member:', memberToDelete.value)
  showConfirmDialog.value = false
  try {
    console.log('Calling delete_member with id:', memberToDelete.value.id)
    await invoke('delete_member', { id: memberToDelete.value.id })
    console.log('Delete successful, reloading data')
    await loadMembers()
    await loadTaloudet() // Päivitä myös taloudet jos jokin poistettiin
  } catch (error) {
    console.error('Virhe poistaessa jäsentä:', error)
    errorMessage.value = (error as any)?.message || error || 'Virhe poistaessa jäsentä'
    showErrorDialog.value = true
  } finally {
    memberToDelete.value = null
  }
}

const cancelDeleteMember = () => {
  showConfirmDialog.value = false
  memberToDelete.value = null
}

const loadTaloudet = async () => {
  try {
    console.log('Loading households with addresses from backend...')
    const householdsData = await invoke('get_households_with_addresses')
    console.log('Received households data:', householdsData)
    
    // Data is already in the right format from backend
    taloudet.value = (householdsData as any[]).map((household: any) => ({
      id: household.id,
      talouden_nimi: household.talouden_nimi || `Talous ${household.id}`,
      osoite: household.osoite,
    }))
    
    console.log('Processed households:', taloudet.value)
  } catch (error) {
    console.error('Virhe ladatessa talouksia:', error)
    taloudet.value = []
  }
}

const loadMembers = async () => {
  try {
    console.log('Loading members from backend...')
    const membersData = await invoke('get_members')
    console.log('Received members data:', membersData)
    
    // Convert backend data to frontend format
    members.value = (membersData as any[]).map((memberWithAddress: any) => ({
      id: memberWithAddress.member.id,
      etunimi: memberWithAddress.member.etunimi,
      sukunimi: memberWithAddress.member.sukunimi,
      henkilotunnus: memberWithAddress.member.henkilotunnus,
      syntymaaika: memberWithAddress.member.syntymaaika,
      puhelinnumero: memberWithAddress.member.puhelinnumero,
      sahkoposti: memberWithAddress.member.sahkoposti,
      liittymispaiva: memberWithAddress.member.liittymispaiva,
      jasentyyppi: memberWithAddress.member.jasentyyppi,
      aktiivinen: memberWithAddress.member.aktiivinen,
      // Add address and household info for future use
      katuosoite: memberWithAddress.address.katuosoite,
      postinumero: memberWithAddress.address.postinumero,
      postitoimipaikka: memberWithAddress.address.postitoimipaikka,
      talouden_nimi: memberWithAddress.household.talouden_nimi,
    }))
    
    console.log('Processed members:', members.value)
  } catch (error) {
    console.error('Virhe ladatessa jäseniä:', error)
    // Fallback to empty array on error
    members.value = []
  }
}

onMounted(() => {
  loadMembers()
  loadTaloudet()
})
</script>