#[doc = "Register `SCU394` reader"]
pub type R = crate::R<Scu394Spec>;
#[doc = "Register `SCU394` writer"]
pub type W = crate::W<Scu394Spec>;
#[doc = "Field `SCUMACTXCLK100MOUTPUTDELAY` reader - SCU_MAC_TXCLK_100M_OUTPUT_DELAY"]
pub type Scumactxclk100moutputdelayR = crate::FieldReader;
#[doc = "Field `SCUMACTXCLK100MOUTPUTDELAY` writer - SCU_MAC_TXCLK_100M_OUTPUT_DELAY"]
pub type Scumactxclk100moutputdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `SCUMACRXCLK100MINPUTDELAY` reader - SCU_MAC_RXCLK_100M_INPUT_DELAY"]
pub type Scumacrxclk100minputdelayR = crate::FieldReader;
#[doc = "Field `SCUMACRXCLK100MINPUTDELAY` writer - SCU_MAC_RXCLK_100M_INPUT_DELAY"]
pub type Scumacrxclk100minputdelayW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `SCUMACRXCLK100MINVERSE` reader - SCU_MAC_RXCLK_100M_INVERSE"]
pub type Scumacrxclk100minverseR = crate::BitReader;
#[doc = "Field `SCUMACRXCLK100MINVERSE` writer - SCU_MAC_RXCLK_100M_INVERSE"]
pub type Scumacrxclk100minverseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - SCU_MAC_TXCLK_100M_OUTPUT_DELAY"]
    #[inline(always)]
    pub fn scumactxclk100moutputdelay(&self) -> Scumactxclk100moutputdelayR {
        Scumactxclk100moutputdelayR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - SCU_MAC_RXCLK_100M_INPUT_DELAY"]
    #[inline(always)]
    pub fn scumacrxclk100minputdelay(&self) -> Scumacrxclk100minputdelayR {
        Scumacrxclk100minputdelayR::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - SCU_MAC_RXCLK_100M_INVERSE"]
    #[inline(always)]
    pub fn scumacrxclk100minverse(&self) -> Scumacrxclk100minverseR {
        Scumacrxclk100minverseR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - SCU_MAC_TXCLK_100M_OUTPUT_DELAY"]
    #[inline(always)]
    pub fn scumactxclk100moutputdelay(&mut self) -> Scumactxclk100moutputdelayW<Scu394Spec> {
        Scumactxclk100moutputdelayW::new(self, 0)
    }
    #[doc = "Bits 12:17 - SCU_MAC_RXCLK_100M_INPUT_DELAY"]
    #[inline(always)]
    pub fn scumacrxclk100minputdelay(&mut self) -> Scumacrxclk100minputdelayW<Scu394Spec> {
        Scumacrxclk100minputdelayW::new(self, 12)
    }
    #[doc = "Bit 24 - SCU_MAC_RXCLK_100M_INVERSE"]
    #[inline(always)]
    pub fn scumacrxclk100minverse(&mut self) -> Scumacrxclk100minverseW<Scu394Spec> {
        Scumacrxclk100minverseW::new(self, 24)
    }
}
#[doc = "MAC0/1 Interface Clock Delay 100M Setting\n\nYou can [`read`](crate::Reg::read) this register and get [`scu394::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu394::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu394Spec;
impl crate::RegisterSpec for Scu394Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu394::R`](R) reader structure"]
impl crate::Readable for Scu394Spec {}
#[doc = "`write(|w| ..)` method takes [`scu394::W`](W) writer structure"]
impl crate::Writable for Scu394Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU394 to value 0"]
impl crate::Resettable for Scu394Spec {}
