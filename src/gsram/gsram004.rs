#[doc = "Register `GSRAM004` reader"]
pub type R = crate::R<Gsram004Spec>;
#[doc = "Register `GSRAM004` writer"]
pub type W = crate::W<Gsram004Spec>;
#[doc = "Field `CPU0RSTB` reader - CPU0_RSTB"]
pub type Cpu0rstbR = crate::BitReader;
#[doc = "Field `CPU1RSTB` reader - CPU1_RSTB"]
pub type Cpu1rstbR = crate::BitReader;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `CPU0EN` reader - CPU0_EN"]
pub type Cpu0enR = crate::BitReader;
#[doc = "Field `CPU0EN` writer - CPU0_EN"]
pub type Cpu0enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPU1EN` reader - CPU1_EN"]
pub type Cpu1enR = crate::BitReader;
#[doc = "Field `CPU1EN` writer - CPU1_EN"]
pub type Cpu1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU0_RSTB"]
    #[inline(always)]
    pub fn cpu0rstb(&self) -> Cpu0rstbR {
        Cpu0rstbR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU1_RSTB"]
    #[inline(always)]
    pub fn cpu1rstb(&self) -> Cpu1rstbR {
        Cpu1rstbR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - CPU0_EN"]
    #[inline(always)]
    pub fn cpu0en(&self) -> Cpu0enR {
        Cpu0enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU1_EN"]
    #[inline(always)]
    pub fn cpu1en(&self) -> Cpu1enR {
        Cpu1enR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - CPU0_EN"]
    #[inline(always)]
    pub fn cpu0en(&mut self) -> Cpu0enW<Gsram004Spec> {
        Cpu0enW::new(self, 4)
    }
    #[doc = "Bit 5 - CPU1_EN"]
    #[inline(always)]
    pub fn cpu1en(&mut self) -> Cpu1enW<Gsram004Spec> {
        Cpu1enW::new(self, 5)
    }
}
#[doc = "GSRAM\\_INFO\n\nYou can [`read`](crate::Reg::read) this register and get [`gsram004::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gsram004::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gsram004Spec;
impl crate::RegisterSpec for Gsram004Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsram004::R`](R) reader structure"]
impl crate::Readable for Gsram004Spec {}
#[doc = "`write(|w| ..)` method takes [`gsram004::W`](W) writer structure"]
impl crate::Writable for Gsram004Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GSRAM004 to value 0x03"]
impl crate::Resettable for Gsram004Spec {
    const RESET_VALUE: u32 = 0x03;
}
