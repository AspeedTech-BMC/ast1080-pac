#[doc = "Register `SCU100` reader"]
pub type R = crate::R<Scu100Spec>;
#[doc = "Register `SCU100` writer"]
pub type W = crate::W<Scu100Spec>;
#[doc = "Field `SCULPCRSTFALLINGSTS` reader - SCU_LPC_RST_FALLING_STS"]
pub type SculpcrstfallingstsR = crate::BitReader;
#[doc = "Field `SCULPCRSTFALLINGSTS` writer - SCU_LPC_RST_FALLING_STS"]
pub type SculpcrstfallingstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCULPCRSTRISINGSTS` reader - SCU_LPC_RST_RISING_STS"]
pub type SculpcrstrisingstsR = crate::BitReader;
#[doc = "Field `SCULPCRSTRISINGSTS` writer - SCU_LPC_RST_RISING_STS"]
pub type SculpcrstrisingstsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `SCULPCRSTCNT` reader - SCU_LPC_RST_CNT"]
pub type SculpcrstcntR = crate::FieldReader;
#[doc = "Field `SCULPCRSTCNTCLR` reader - SCU_LPC_RST_CNT_CLR"]
pub type SculpcrstcntclrR = crate::BitReader;
#[doc = "Field `SCULPCRSTCNTCLR` writer - SCU_LPC_RST_CNT_CLR"]
pub type SculpcrstcntclrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_LPC_RST_FALLING_STS"]
    #[inline(always)]
    pub fn sculpcrstfallingsts(&self) -> SculpcrstfallingstsR {
        SculpcrstfallingstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_LPC_RST_RISING_STS"]
    #[inline(always)]
    pub fn sculpcrstrisingsts(&self) -> SculpcrstrisingstsR {
        SculpcrstrisingstsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:15 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:19 - SCU_LPC_RST_CNT"]
    #[inline(always)]
    pub fn sculpcrstcnt(&self) -> SculpcrstcntR {
        SculpcrstcntR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - SCU_LPC_RST_CNT_CLR"]
    #[inline(always)]
    pub fn sculpcrstcntclr(&self) -> SculpcrstcntclrR {
        SculpcrstcntclrR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_LPC_RST_FALLING_STS"]
    #[inline(always)]
    pub fn sculpcrstfallingsts(&mut self) -> SculpcrstfallingstsW<Scu100Spec> {
        SculpcrstfallingstsW::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_LPC_RST_RISING_STS"]
    #[inline(always)]
    pub fn sculpcrstrisingsts(&mut self) -> SculpcrstrisingstsW<Scu100Spec> {
        SculpcrstrisingstsW::new(self, 1)
    }
    #[doc = "Bit 20 - SCU_LPC_RST_CNT_CLR"]
    #[inline(always)]
    pub fn sculpcrstcntclr(&mut self) -> SculpcrstcntclrW<Scu100Spec> {
        SculpcrstcntclrW::new(self, 20)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu100::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu100::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu100Spec;
impl crate::RegisterSpec for Scu100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu100::R`](R) reader structure"]
impl crate::Readable for Scu100Spec {}
#[doc = "`write(|w| ..)` method takes [`scu100::W`](W) writer structure"]
impl crate::Writable for Scu100Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU100 to value 0"]
impl crate::Resettable for Scu100Spec {}
