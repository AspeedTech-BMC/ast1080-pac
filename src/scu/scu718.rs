#[doc = "Register `SCU718` reader"]
pub type R = crate::R<Scu718Spec>;
#[doc = "Register `SCU718` writer"]
pub type W = crate::W<Scu718Spec>;
#[doc = "Field `SCUMUXLOCKIO192` reader - SCU_MUX_LOCK_IO192"]
pub type Scumuxlockio192R = crate::BitReader;
#[doc = "Field `SCUMUXLOCKIO192` writer - SCU_MUX_LOCK_IO192"]
pub type Scumuxlockio192W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCUMUXLOCKIO193` reader - SCU_MUX_LOCK_IO193"]
pub type Scumuxlockio193R = crate::BitReader;
#[doc = "Field `SCUMUXLOCKIO193` writer - SCU_MUX_LOCK_IO193"]
pub type Scumuxlockio193W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_MUX_LOCK_IO192"]
    #[inline(always)]
    pub fn scumuxlockio192(&self) -> Scumuxlockio192R {
        Scumuxlockio192R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCU_MUX_LOCK_IO193"]
    #[inline(always)]
    pub fn scumuxlockio193(&self) -> Scumuxlockio193R {
        Scumuxlockio193R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_MUX_LOCK_IO192"]
    #[inline(always)]
    pub fn scumuxlockio192(&mut self) -> Scumuxlockio192W<Scu718Spec> {
        Scumuxlockio192W::new(self, 0)
    }
    #[doc = "Bit 1 - SCU_MUX_LOCK_IO193"]
    #[inline(always)]
    pub fn scumuxlockio193(&mut self) -> Scumuxlockio193W<Scu718Spec> {
        Scumuxlockio193W::new(self, 1)
    }
}
#[doc = "IO Lock Register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`scu718::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu718::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu718Spec;
impl crate::RegisterSpec for Scu718Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu718::R`](R) reader structure"]
impl crate::Readable for Scu718Spec {}
#[doc = "`write(|w| ..)` method takes [`scu718::W`](W) writer structure"]
impl crate::Writable for Scu718Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU718 to value 0"]
impl crate::Resettable for Scu718Spec {}
