import jsPDF from 'jspdf'
import JsBarcode from 'jsbarcode'

export interface VectorInvoiceData {
  invoice: any
  organization: any
}

const createInvoicePDF = (data: VectorInvoiceData): jsPDF => {
  const pdf = new jsPDF('p', 'mm', 'a4')
  const { invoice, organization } = data

  // Set up margins and positions
  const margin = 20
  let yPos = margin

  // Header - Organization info
  pdf.setFontSize(18)
  pdf.setFont('helvetica', 'bold')
  pdf.text(organization?.nimi || 'Yhdistys', margin, yPos)

  // Invoice title on same level as organization name
  pdf.setFontSize(16)
  pdf.setFont('helvetica', 'bold')
  pdf.text('LASKU', 105, yPos, { align: 'center' })

  yPos += 8

  pdf.setFontSize(12)
  pdf.setFont('helvetica', 'normal')
  if (organization?.katuosoite) {
    pdf.text(organization.katuosoite, margin, yPos)
    yPos += 5
  }
  if (organization?.postinumero && organization?.postitoimipaikka) {
    pdf.text(`${organization.postinumero} ${organization.postitoimipaikka}`, margin, yPos)
    yPos += 5
  }

  // Contact info
  pdf.setFontSize(10)
  if (organization?.puhelinnumero) {
    pdf.text(`Puh: ${organization.puhelinnumero}`, margin, yPos)
    yPos += 4
  }
  if (organization?.sahkoposti) {
    pdf.text(organization.sahkoposti, margin, yPos)
    yPos += 4
  }
  if (organization?.y_tunnus) {
    pdf.text(`Y-tunnus: ${organization.y_tunnus}`, margin, yPos)
    yPos += 4
  }

  // Line separator
  yPos += 10
  pdf.setLineWidth(1)
  pdf.line(margin, yPos, 190, yPos)
  yPos += 15

  // Recipient info directly under organization info
  pdf.setFontSize(12)
  pdf.setFont('helvetica', 'bold')
  pdf.text('Laskun saaja', margin, yPos)
  yPos += 7

  // Recipient info without box
  pdf.setFont('helvetica', 'bold')
  pdf.text(invoice?.household?.vastaanottaja || invoice?.household?.talouden_nimi || 'Nimetön', margin, yPos)
  yPos += 5
  pdf.setFont('helvetica', 'normal')
  if (invoice?.address?.katuosoite) {
    pdf.text(invoice.address.katuosoite, margin, yPos)
    yPos += 5
  }
  if (invoice?.address?.postinumero && invoice?.address?.postitoimipaikka) {
    pdf.text(`${invoice.address.postinumero} ${invoice.address.postitoimipaikka}`, margin, yPos)
    yPos += 5
  }

  yPos += 15

  // Invoice details on the right side
  const rightCol = 110
  let rightY = 30  // Position higher up
  const spacing = 50

  pdf.setFontSize(12)
  pdf.setFont('helvetica', 'bold')
  pdf.text('Laskun numero:', rightCol, rightY)
  pdf.setFont('helvetica', 'normal')
  pdf.text(String(invoice?.id || ''), rightCol + spacing, rightY)
  rightY += 7

  pdf.setFont('helvetica', 'bold')
  pdf.text('Laskun päivämäärä:', rightCol, rightY)
  pdf.setFont('helvetica', 'normal')
  pdf.text(formatDate(invoice?.luontipaiva), rightCol + spacing, rightY)
  rightY += 7

  pdf.setFont('helvetica', 'bold')
  pdf.text('Eräpäivä:', rightCol, rightY)
  pdf.setFont('helvetica', 'normal')
  pdf.text(formatDate(invoice?.erapaiva), rightCol + spacing, rightY)
  rightY += 7

  pdf.setFont('helvetica', 'bold')
  pdf.text('Viitenumero:', rightCol, rightY)
  pdf.setFont('helvetica', 'normal')
  pdf.text(String(invoice?.viitenumero || ''), rightCol + spacing, rightY)

  yPos = Math.max(yPos, rightY) + 10

  // Table header
  const tableY = yPos
  const colWidths = [80, 60, 30]
  const colPositions = [margin, margin + colWidths[0], margin + colWidths[0] + colWidths[1]]

  pdf.setFillColor(245, 245, 245)
  pdf.rect(margin, tableY, 170, 8, 'F')

  pdf.setFont('helvetica', 'bold')
  pdf.text('Kuvaus', colPositions[0] + 2, tableY + 6)
  pdf.text('Jäsen', colPositions[1] + 2, tableY + 6)
  pdf.text('Summa', colPositions[2] + 2, tableY + 6)

  // Table borders
  pdf.setLineWidth(0.5)
  pdf.rect(margin, tableY, 170, 8)

  let tableRowY = tableY + 8
  let totalAmount = 0

  // Table rows
  if (invoice?.lines) {
    pdf.setFont('helvetica', 'normal')
    for (const line of invoice.lines) {
      pdf.text(line.line.kuvaus || '', colPositions[0] + 2, tableRowY + 6)
      pdf.text(`${line.member.etunimi} ${line.member.sukunimi}`, colPositions[1] + 2, tableRowY + 6)
      const amount = line.line.summa || 0
      pdf.text(`${amount.toFixed(2)} €`, colPositions[2] + 2, tableRowY + 6)

      pdf.rect(margin, tableRowY, 170, 8)
      tableRowY += 8
      totalAmount += amount
    }
  }

  // Total row
  pdf.setFillColor(240, 240, 240)
  pdf.rect(margin, tableRowY, 170, 8, 'F')
  pdf.setFont('helvetica', 'bold')
  pdf.text('Yhteensä:', colPositions[1] + 2, tableRowY + 6)
  pdf.text(`${(invoice?.summa || totalAmount).toFixed(2)} €`, colPositions[2] + 2, tableRowY + 6)
  pdf.rect(margin, tableRowY, 170, 8)

  yPos = tableRowY + 20

  // Payment info
  pdf.setFont('helvetica', 'bold')
  pdf.text('Maksutiedot', margin, yPos)
  yPos += 10

  pdf.setFont('helvetica', 'normal')
  pdf.text(`Saajan tilinumero: ${organization?.pankkitili || 'FI0000000000000000'}`, margin, yPos)
  yPos += 6
  if (organization?.bic) {
    pdf.text(`BIC: ${organization.bic}`, margin, yPos)
    yPos += 6
  }
  pdf.text(`Viitenumero: ${invoice?.viitenumero || ''}`, margin, yPos)
  yPos += 6
  pdf.text(`Eräpäivä: ${formatDate(invoice?.erapaiva)}`, margin, yPos)
  yPos += 6
  pdf.setFont('helvetica', 'bold')
  pdf.text(`Maksettava summa: ${(invoice?.summa || 0).toFixed(2)} €`, margin, yPos)
  yPos += 15

  // Generate barcode vector data
  const barcodeData = generateBarcodeString(invoice, organization)
  if (barcodeData) {
    // Create barcode as vector
    const canvas = document.createElement('canvas')
    JsBarcode(canvas, barcodeData, {
      format: "CODE128",
      width: 2,
      height: 40,
      displayValue: false,
      margin: 0
    })

    // Add barcode to PDF
    pdf.setFillColor(249, 249, 249)
    pdf.rect(margin, yPos, 170, 20, 'F')
    pdf.setFont('helvetica', 'bold')
    pdf.text('Pankkiviivakoodi:', margin + 2, yPos + 5)

    // Convert canvas to image and add to PDF
    const imgData = canvas.toDataURL('image/png')
    pdf.addImage(imgData, 'PNG', margin + 2, yPos + 8, 100, 10)

    yPos += 25
  }

  // Footer
  pdf.setFontSize(10)
  pdf.setFont('helvetica', 'normal')
  pdf.text('Kiitos jäsenyydestäsi!', 105, yPos + 10, { align: 'center' })
  if (organization?.sahkoposti || organization?.puhelinnumero) {
    const contact = organization?.sahkoposti || organization?.puhelinnumero
    pdf.text(`Laskua koskevissa kysymyksissä ottakaa yhteyttä: ${contact}`, 105, yPos + 15, { align: 'center' })
  }

  return pdf
}

export const generatePrintablePDF = async (data: VectorInvoiceData): Promise<ArrayBuffer> => {
  const pdf = createInvoicePDF(data)
  return pdf.output('arraybuffer')
}

export const generateVectorInvoicePDF = async (data: VectorInvoiceData, defaultFilename: string): Promise<void> => {
  try {
    const { invoke } = await import('@tauri-apps/api/core')
    const savePath = await invoke('show_save_dialog', { defaultFilename })

    if (!savePath) return

    const pdf = createInvoicePDF(data)

    // Save PDF
    const pdfData = pdf.output('arraybuffer')
    const uint8Array = Array.from(new Uint8Array(pdfData))
    await invoke('save_pdf_file', { filePath: savePath, data: uint8Array })

  } catch (error) {
    console.error('Vector PDF generation failed:', error)
    throw new Error(`Vector PDF luonti epäonnistui: ${error instanceof Error ? error.message : String(error)}`)
  }
}

function formatDate(dateString: string): string {
  if (!dateString) return ''
  return new Date(dateString).toLocaleDateString('fi-FI')
}

function generateBarcodeString(invoice: any, organization: any): string {
  if (!invoice || !organization) return ''

  try {
    console.log('=== VECTOR PDF BARCODE GENERATION ===')

    // Use correct 54-character logic
    const iban = (organization.pankkitili || 'FI1410093000123458').replace(/\s/g, '').toUpperCase()
    console.log('IBAN:', iban)

    // Process IBAN to get exactly 16 digits (for 54-char total)
    let ibanWithoutCountry
    if (iban.startsWith('FI') && iban.length === 18) {
      ibanWithoutCountry = iban.slice(2) // Remove FI -> 16 digits
    } else {
      ibanWithoutCountry = '1410093000123458' // 16 digits fallback
    }

    if (ibanWithoutCountry.length !== 16) {
      console.warn('IBAN part wrong length:', ibanWithoutCountry.length, 'adjusting to 16')
      if (ibanWithoutCountry.length > 16) {
        ibanWithoutCountry = ibanWithoutCountry.slice(-16)
      } else {
        ibanWithoutCountry = ibanWithoutCountry.padStart(16, '0')
      }
    }

    const amount = invoice.summa || 0
    const safeAmount = Math.min(Math.max(Math.abs(amount), 0), 999999.99)
    const amountCents = Math.round(safeAmount * 100)
    const formattedAmount = amountCents.toString().padStart(8, '0')

    let reference = (invoice.viitenumero || '1').replace(/\s/g, '')
    const paddedReference = reference.padStart(20, '0')

    let dueDate = '000000'
    if (invoice.erapaiva) {
      const date = new Date(invoice.erapaiva)
      if (!isNaN(date.getTime())) {
        const yy = date.getFullYear().toString().slice(-2)
        const mm = (date.getMonth() + 1).toString().padStart(2, '0')
        const dd = date.getDate().toString().padStart(2, '0')
        dueDate = yy + mm + dd
      }
    }

    // Generate 54-character barcode
    const barcode = `4${ibanWithoutCountry}${formattedAmount}000${paddedReference}${dueDate}`

    console.log('Vector PDF barcode components:', {
      version: '4',
      iban: ibanWithoutCountry,
      amount: formattedAmount,
      reference: paddedReference,
      dueDate: dueDate,
      fullBarcode: barcode,
      length: barcode.length
    })

    return barcode.length === 54 ? barcode : ''

  } catch (error) {
    console.error('Error generating barcode string:', error)
    return ''
  }
}