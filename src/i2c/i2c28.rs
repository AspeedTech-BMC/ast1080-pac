#[doc = "Register `I2C28` reader"]
pub type R = crate::R<I2c28Spec>;
#[doc = "Register `I2C28` writer"]
pub type W = crate::W<I2c28Spec>;
#[doc = "Field `SCMDTXD` reader - SCMD_TXD"]
pub type ScmdtxdR = crate::BitReader;
#[doc = "Field `SCMDTXD` writer - SCMD_TXD"]
pub type ScmdtxdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `SCMDRXLAST` reader - SCMD_RXLAST"]
pub type ScmdrxlastR = crate::BitReader;
#[doc = "Field `SCMDRXLAST` writer - SCMD_RXLAST"]
pub type ScmdrxlastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `SBUFTXEN` reader - SBUF_TX_EN"]
pub type SbuftxenR = crate::BitReader;
#[doc = "Field `SBUFTXEN` writer - SBUF_TX_EN"]
pub type SbuftxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBUFRXEN` reader - SBUF_RX_EN"]
pub type SbufrxenR = crate::BitReader;
#[doc = "Field `SBUFRXEN` writer - SBUF_RX_EN"]
pub type SbufrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STXEN` reader - STX_EN"]
pub type StxenR = crate::BitReader;
#[doc = "Field `STXEN` writer - STX_EN"]
pub type StxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRXEN` reader - SRX_EN"]
pub type SrxenR = crate::BitReader;
#[doc = "Field `SRXEN` writer - SRX_EN"]
pub type SrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SALERTEN` reader - SALERT_EN"]
pub type SalertenR = crate::BitReader;
#[doc = "Field `SALERTEN` writer - SALERT_EN"]
pub type SalertenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SPKGEN` reader - SPKG_EN"]
pub type SpkgenR = crate::BitReader;
#[doc = "Field `SPKGEN` writer - SPKG_EN"]
pub type SpkgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAUTONACK` reader - SAUTO_NACK"]
pub type SautonackR = crate::FieldReader;
#[doc = "Field `SAUTONACK` writer - SAUTO_NACK"]
pub type SautonackW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - SCMD_TXD"]
    #[inline(always)]
    pub fn scmdtxd(&self) -> ScmdtxdR {
        ScmdtxdR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SCMD_RXLAST"]
    #[inline(always)]
    pub fn scmdrxlast(&self) -> ScmdrxlastR {
        ScmdrxlastR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SBUF_TX_EN"]
    #[inline(always)]
    pub fn sbuftxen(&self) -> SbuftxenR {
        SbuftxenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SBUF_RX_EN"]
    #[inline(always)]
    pub fn sbufrxen(&self) -> SbufrxenR {
        SbufrxenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - STX_EN"]
    #[inline(always)]
    pub fn stxen(&self) -> StxenR {
        StxenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SRX_EN"]
    #[inline(always)]
    pub fn srxen(&self) -> SrxenR {
        SrxenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SALERT_EN"]
    #[inline(always)]
    pub fn salerten(&self) -> SalertenR {
        SalertenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - SPKG_EN"]
    #[inline(always)]
    pub fn spkgen(&self) -> SpkgenR {
        SpkgenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - SAUTO_NACK"]
    #[inline(always)]
    pub fn sautonack(&self) -> SautonackR {
        SautonackR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - SCMD_TXD"]
    #[inline(always)]
    pub fn scmdtxd(&mut self) -> ScmdtxdW<I2c28Spec> {
        ScmdtxdW::new(self, 2)
    }
    #[doc = "Bit 4 - SCMD_RXLAST"]
    #[inline(always)]
    pub fn scmdrxlast(&mut self) -> ScmdrxlastW<I2c28Spec> {
        ScmdrxlastW::new(self, 4)
    }
    #[doc = "Bit 6 - SBUF_TX_EN"]
    #[inline(always)]
    pub fn sbuftxen(&mut self) -> SbuftxenW<I2c28Spec> {
        SbuftxenW::new(self, 6)
    }
    #[doc = "Bit 7 - SBUF_RX_EN"]
    #[inline(always)]
    pub fn sbufrxen(&mut self) -> SbufrxenW<I2c28Spec> {
        SbufrxenW::new(self, 7)
    }
    #[doc = "Bit 8 - STX_EN"]
    #[inline(always)]
    pub fn stxen(&mut self) -> StxenW<I2c28Spec> {
        StxenW::new(self, 8)
    }
    #[doc = "Bit 9 - SRX_EN"]
    #[inline(always)]
    pub fn srxen(&mut self) -> SrxenW<I2c28Spec> {
        SrxenW::new(self, 9)
    }
    #[doc = "Bit 10 - SALERT_EN"]
    #[inline(always)]
    pub fn salerten(&mut self) -> SalertenW<I2c28Spec> {
        SalertenW::new(self, 10)
    }
    #[doc = "Bit 16 - SPKG_EN"]
    #[inline(always)]
    pub fn spkgen(&mut self) -> SpkgenW<I2c28Spec> {
        SpkgenW::new(self, 16)
    }
    #[doc = "Bits 24:31 - SAUTO_NACK"]
    #[inline(always)]
    pub fn sautonack(&mut self) -> SautonackW<I2c28Spec> {
        SautonackW::new(self, 24)
    }
}
#[doc = "Slave Command/Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2c28Spec;
impl crate::RegisterSpec for I2c28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c28::R`](R) reader structure"]
impl crate::Readable for I2c28Spec {}
#[doc = "`write(|w| ..)` method takes [`i2c28::W`](W) writer structure"]
impl crate::Writable for I2c28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C28 to value 0"]
impl crate::Resettable for I2c28Spec {}
