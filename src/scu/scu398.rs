#[doc = "Register `SCU398` reader"]
pub type R = crate::R<Scu398Spec>;
#[doc = "Register `SCU398` writer"]
pub type W = crate::W<Scu398Spec>;
#[doc = "Field `SCUMACTXCLK10MOUTPUTDELAY` reader - SCU_MAC_TXCLK_10M_OUTPUT_DELAY"]
pub type Scumactxclk10moutputdelayR = crate::FieldReader;
#[doc = "Field `SCUMACTXCLK10MOUTPUTDELAY` writer - SCU_MAC_TXCLK_10M_OUTPUT_DELAY"]
pub type Scumactxclk10moutputdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUMACRXCLK10MINPUTDELAY` reader - SCU_MAC_RXCLK_10M_INPUT_DELAY"]
pub type Scumacrxclk10minputdelayR = crate::FieldReader;
#[doc = "Field `SCUMACRXCLK10MINPUTDELAY` writer - SCU_MAC_RXCLK_10M_INPUT_DELAY"]
pub type Scumacrxclk10minputdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUMACRXCLK10MINVERSE` reader - SCU_MAC_RXCLK_10M_INVERSE"]
pub type Scumacrxclk10minverseR = crate::BitReader;
#[doc = "Field `SCUMACRXCLK10MINVERSE` writer - SCU_MAC_RXCLK_10M_INVERSE"]
pub type Scumacrxclk10minverseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - SCU_MAC_TXCLK_10M_OUTPUT_DELAY"]
    #[inline(always)]
    pub fn scumactxclk10moutputdelay(&self) -> Scumactxclk10moutputdelayR {
        Scumactxclk10moutputdelayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - SCU_MAC_RXCLK_10M_INPUT_DELAY"]
    #[inline(always)]
    pub fn scumacrxclk10minputdelay(&self) -> Scumacrxclk10minputdelayR {
        Scumacrxclk10minputdelayR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - SCU_MAC_RXCLK_10M_INVERSE"]
    #[inline(always)]
    pub fn scumacrxclk10minverse(&self) -> Scumacrxclk10minverseR {
        Scumacrxclk10minverseR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SCU_MAC_TXCLK_10M_OUTPUT_DELAY"]
    #[inline(always)]
    pub fn scumactxclk10moutputdelay(&mut self) -> Scumactxclk10moutputdelayW<Scu398Spec> {
        Scumactxclk10moutputdelayW::new(self, 0)
    }
    #[doc = "Bits 12:17 - SCU_MAC_RXCLK_10M_INPUT_DELAY"]
    #[inline(always)]
    pub fn scumacrxclk10minputdelay(&mut self) -> Scumacrxclk10minputdelayW<Scu398Spec> {
        Scumacrxclk10minputdelayW::new(self, 12)
    }
    #[doc = "Bit 24 - SCU_MAC_RXCLK_10M_INVERSE"]
    #[inline(always)]
    pub fn scumacrxclk10minverse(&mut self) -> Scumacrxclk10minverseW<Scu398Spec> {
        Scumacrxclk10minverseW::new(self, 24)
    }
}
#[doc = "MAC0/1 Interface Clock Delay 10M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu398::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu398::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu398Spec;
impl crate::RegisterSpec for Scu398Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu398::R`](R) reader structure"]
impl crate::Readable for Scu398Spec {}
#[doc = "`write(|w| ..)` method takes [`scu398::W`](W) writer structure"]
impl crate::Writable for Scu398Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU398 to value 0"]
impl crate::Resettable for Scu398Spec {}
