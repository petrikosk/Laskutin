import jsPDF from 'jspdf'
import html2canvas from 'html2canvas'

export interface InvoiceData {
  invoice: any
  organization: any
}

export const generateInvoicePDF = async (invoiceElement: HTMLElement, defaultFilename: string): Promise<void> => {
  try {
    // Luo canvas elementistä
    const canvas = await html2canvas(invoiceElement, {
      scale: 2, // Parempi laatu
      useCORS: true,
      allowTaint: true,
      backgroundColor: '#ffffff',
      width: 794, // A4 leveys pikseleinä (210mm * 3.78)
      height: invoiceElement.offsetHeight, // Käytä elementin todellista korkeutta
      windowWidth: 794,
      windowHeight: invoiceElement.offsetHeight
    })

    // Laske PDF:n mitat (A4-koko)
    const imgWidth = 210 // A4 leveys mm
    const pageHeight = 297 // A4 korkeus mm
    const imgHeight = (canvas.height * imgWidth) / canvas.width

    // Luo PDF
    const pdf = new jsPDF('p', 'mm', 'a4')

    // Jos kuva mahtuu yhdelle sivulle, älä lisää sivuja
    if (imgHeight <= pageHeight) {
      pdf.addImage(canvas.toDataURL('image/png'), 'PNG', 0, 0, imgWidth, imgHeight)
    } else {
      // Monisivuinen lasku
      let heightLeft = imgHeight
      let position = 0

      // Lisää ensimmäinen sivu
      pdf.addImage(canvas.toDataURL('image/png'), 'PNG', 0, position, imgWidth, imgHeight)
      heightLeft -= pageHeight

      // Lisää lisäsivuja tarvittaessa
      while (heightLeft > 0) {
        position = heightLeft - imgHeight
        pdf.addPage()
        pdf.addImage(canvas.toDataURL('image/png'), 'PNG', 0, position, imgWidth, imgHeight)
        heightLeft -= pageHeight
      }
    }

    // Tallenna selaimessa (fallback)
    pdf.save(defaultFilename)
  } catch (error) {
    console.error('Virhe PDF:n luonnissa:', error)
    throw new Error('PDF:n luonti epäonnistui')
  }
}

export const generateAndSavePDF = async (invoiceElement: HTMLElement, defaultFilename: string): Promise<void> => {
  try {
    // Pyydä tallennuspolkua backend-komennolla
    const { invoke } = await import('@tauri-apps/api/core')
    const savePath = await invoke('show_save_dialog', { defaultFilename })
    
    if (!savePath) {
      // Käyttäjä perui tallennuksen
      return
    }

    // Luo canvas elementistä
    const canvas = await html2canvas(invoiceElement, {
      scale: 2,
      useCORS: true,
      allowTaint: true,
      backgroundColor: '#ffffff',
      width: 794, // A4 leveys pikseleinä (210mm * 3.78)
      height: invoiceElement.offsetHeight, // Käytä elementin todellista korkeutta
      windowWidth: 794,
      windowHeight: invoiceElement.offsetHeight
    })

    // Laske PDF:n mitat (A4-koko)
    const imgWidth = 210
    const pageHeight = 297
    const imgHeight = (canvas.height * imgWidth) / canvas.width

    // Luo PDF
    const pdf = new jsPDF('p', 'mm', 'a4')

    // Jos kuva mahtuu yhdelle sivulle, älä lisää sivuja
    if (imgHeight <= pageHeight) {
      pdf.addImage(canvas.toDataURL('image/png'), 'PNG', 0, 0, imgWidth, imgHeight)
    } else {
      // Monisivuinen lasku
      let heightLeft = imgHeight
      let position = 0

      // Lisää ensimmäinen sivu
      pdf.addImage(canvas.toDataURL('image/png'), 'PNG', 0, position, imgWidth, imgHeight)
      heightLeft -= pageHeight

      // Lisää lisäsivuja tarvittaessa
      while (heightLeft > 0) {
        position = heightLeft - imgHeight
        pdf.addPage()
        pdf.addImage(canvas.toDataURL('image/png'), 'PNG', 0, position, imgWidth, imgHeight)
        heightLeft -= pageHeight
      }
    }

    // Muunna PDF uint8array:ksi
    const pdfData = pdf.output('arraybuffer')
    const uint8Array = Array.from(new Uint8Array(pdfData))

    // Tallenna tiedosto backend-komennolla
    await invoke('save_pdf_file', { filePath: savePath, data: uint8Array })
    
  } catch (error) {
    console.error('Virhe PDF:n luonnissa:', error)
    throw new Error('PDF:n luonti epäonnistui')
  }
}

export const printInvoice = async (invoiceElement: HTMLElement): Promise<void> => {
  try {
    // Luo tulostustyylinen versio elementistä
    const printContent = createPrintableContent(invoiceElement)
    
    // Lisää tulostustyylitiedot sivulle
    const styleElement = document.createElement('style')
    styleElement.textContent = `
      @media print {
        body * { visibility: hidden; }
        .print-content, .print-content * { visibility: visible; }
        .print-content { 
          position: absolute;
          left: 0;
          top: 0;
          width: 100%;
        }
        ${getInvoiceStyles()}
      }
    `
    document.head.appendChild(styleElement)
    
    // Lisää tulostussisältö sivulle
    const printDiv = document.createElement('div')
    printDiv.className = 'print-content'
    printDiv.innerHTML = printContent
    printDiv.style.position = 'absolute'
    printDiv.style.left = '-9999px'
    printDiv.style.top = '-9999px'
    document.body.appendChild(printDiv)
    
    // Tulosta
    window.print()
    
    // Siivoa
    document.head.removeChild(styleElement)
    document.body.removeChild(printDiv)
    
  } catch (error) {
    console.error('Virhe tulostuksessa:', error)
    throw new Error('Tulostus epäonnistui')
  }
}

const createPrintableContent = (element: HTMLElement): string => {
  return `
    <!DOCTYPE html>
    <html>
    <head>
      <title>Lasku</title>
      <style>
        ${getInvoiceStyles()}
      </style>
    </head>
    <body>
      ${element.outerHTML}
    </body>
    </html>
  `
}

// Laskun tyylitiedot tulostusta varten
const getInvoiceStyles = (): string => {
  return `
    .invoice-pdf {
      background: white;
      color: black;
      font-family: Arial, sans-serif;
      font-size: 12px;
      line-height: 1.4;
    }

    .invoice-container {
      max-width: 210mm;
      min-height: 297mm;
      margin: 0 auto;
      padding: 20mm;
      box-sizing: border-box;
    }

    .invoice-header {
      display: flex;
      justify-content: space-between;
      align-items: flex-start;
      margin-bottom: 30px;
      border-bottom: 2px solid #333;
      padding-bottom: 20px;
    }

    .company-name {
      font-size: 24px;
      font-weight: bold;
      margin: 0 0 10px 0;
      color: #333;
    }

    .address {
      margin-bottom: 10px;
    }

    .contact-info {
      font-size: 11px;
      color: #666;
    }

    .invoice-title h2 {
      font-size: 28px;
      font-weight: bold;
      margin: 0;
      color: #333;
    }

    .invoice-details {
      display: flex;
      justify-content: space-between;
      margin-bottom: 30px;
    }

    .info-table,
    .payment-table {
      border-collapse: collapse;
    }

    .info-table td,
    .payment-table td {
      padding: 5px 10px 5px 0;
      vertical-align: top;
    }

    .recipient-info {
      text-align: right;
    }

    .recipient-info h3 {
      margin: 0 0 10px 0;
      font-size: 14px;
    }

    .recipient-address {
      border: 1px solid #ccc;
      padding: 15px;
      background: #f9f9f9;
      min-width: 200px;
    }

    .invoice-lines {
      margin-bottom: 30px;
    }

    .invoice-lines h3 {
      margin: 0 0 15px 0;
      font-size: 16px;
      border-bottom: 1px solid #ccc;
      padding-bottom: 5px;
    }

    .lines-table {
      width: 100%;
      border-collapse: collapse;
      border: 1px solid #ccc;
    }

    .lines-table th,
    .lines-table td {
      border: 1px solid #ccc;
      padding: 8px;
      text-align: left;
    }

    .lines-table th {
      background: #f5f5f5;
      font-weight: bold;
    }

    .lines-table .amount {
      text-align: right;
      width: 100px;
    }

    .total-row {
      background: #f0f0f0;
      font-weight: bold;
    }

    .payment-info {
      margin-bottom: 30px;
    }

    .payment-info h3 {
      margin: 0 0 15px 0;
      font-size: 16px;
      border-bottom: 1px solid #ccc;
      padding-bottom: 5px;
    }

    .barcode-section {
      margin-bottom: 30px;
      text-align: center;
      border: 1px solid #ccc;
      padding: 15px;
      background: #f9f9f9;
    }

    .barcode-label {
      font-weight: bold;
      margin-bottom: 10px;
    }

    .barcode-svg {
      max-width: 100%;
      height: 50px;
      margin: 5px 0;
      display: block;
    }

    .barcode-svg text {
      display: none !important;
    }

    .additional-info {
      text-align: center;
      color: #666;
      font-size: 11px;
      margin-top: 30px;
    }

    .additional-info p {
      margin: 5px 0;
    }

    @media print {
      .invoice-pdf {
        font-size: 11px;
      }
      
      .invoice-container {
        padding: 15mm;
      }
      
      .company-name {
        font-size: 20px;
      }
      
      .invoice-title h2 {
        font-size: 24px;
      }
    }
  `
}