#[doc = "Register `SPI080` reader"]
pub type R = crate::R<Spi080Spec>;
#[doc = "Register `SPI080` writer"]
pub type W = crate::W<Spi080Spec>;
#[doc = "Field `DMAEN` reader - DMA_EN"]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA_EN"]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMADIRTX` reader - DMA_DIR_TX"]
pub type DmadirtxR = crate::BitReader;
#[doc = "Field `DMADIRTX` writer - DMA_DIR_TX"]
pub type DmadirtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACHKSUMONLY` reader - DMA_CHKSUM_ONLY"]
pub type DmachksumonlyR = crate::BitReader;
#[doc = "Field `DMACHKSUMONLY` writer - DMA_CHKSUM_ONLY"]
pub type DmachksumonlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMACBR` reader - DMA_CBR"]
pub type DmacbrR = crate::BitReader;
#[doc = "Field `DMACBR` writer - DMA_CBR"]
pub type DmacbrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMABUFMODE` reader - DMA_BUF_MODE"]
pub type DmabufmodeR = crate::BitReader;
#[doc = "Field `DMABUFMODE` writer - DMA_BUF_MODE"]
pub type DmabufmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `CBRDLY` reader - CBR_DLY"]
pub type CbrdlyR = crate::FieldReader;
#[doc = "Field `CBRDLY` writer - CBR_DLY"]
pub type CbrdlyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CBRCLKRATE` reader - CBR_CLK_RATE"]
pub type CbrclkrateR = crate::FieldReader;
#[doc = "Field `CBRCLKRATE` writer - CBR_CLK_RATE"]
pub type CbrclkrateW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - DMA_EN"]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA_DIR_TX"]
    #[inline(always)]
    pub fn dmadirtx(&self) -> DmadirtxR {
        DmadirtxR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA_CHKSUM_ONLY"]
    #[inline(always)]
    pub fn dmachksumonly(&self) -> DmachksumonlyR {
        DmachksumonlyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA_CBR"]
    #[inline(always)]
    pub fn dmacbr(&self) -> DmacbrR {
        DmacbrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA_BUF_MODE"]
    #[inline(always)]
    pub fn dmabufmode(&self) -> DmabufmodeR {
        DmabufmodeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - CBR_DLY"]
    #[inline(always)]
    pub fn cbrdly(&self) -> CbrdlyR {
        CbrdlyR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - CBR_CLK_RATE"]
    #[inline(always)]
    pub fn cbrclkrate(&self) -> CbrclkrateR {
        CbrclkrateR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DMA_EN"]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<Spi080Spec> {
        DmaenW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA_DIR_TX"]
    #[inline(always)]
    pub fn dmadirtx(&mut self) -> DmadirtxW<Spi080Spec> {
        DmadirtxW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA_CHKSUM_ONLY"]
    #[inline(always)]
    pub fn dmachksumonly(&mut self) -> DmachksumonlyW<Spi080Spec> {
        DmachksumonlyW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA_CBR"]
    #[inline(always)]
    pub fn dmacbr(&mut self) -> DmacbrW<Spi080Spec> {
        DmacbrW::new(self, 3)
    }
    #[doc = "Bit 4 - DMA_BUF_MODE"]
    #[inline(always)]
    pub fn dmabufmode(&mut self) -> DmabufmodeW<Spi080Spec> {
        DmabufmodeW::new(self, 4)
    }
    #[doc = "Bits 8:15 - CBR_DLY"]
    #[inline(always)]
    pub fn cbrdly(&mut self) -> CbrdlyW<Spi080Spec> {
        CbrdlyW::new(self, 8)
    }
    #[doc = "Bits 16:19 - CBR_CLK_RATE"]
    #[inline(always)]
    pub fn cbrclkrate(&mut self) -> CbrclkrateW<Spi080Spec> {
        CbrclkrateW::new(self, 16)
    }
}
#[doc = "DMA Control/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi080::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi080::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spi080Spec;
impl crate::RegisterSpec for Spi080Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi080::R`](R) reader structure"]
impl crate::Readable for Spi080Spec {}
#[doc = "`write(|w| ..)` method takes [`spi080::W`](W) writer structure"]
impl crate::Writable for Spi080Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI080 to value 0"]
impl crate::Resettable for Spi080Spec {}
