#[doc = "Register `SCUE5C` reader"]
pub type R = crate::R<Scue5cSpec>;
#[doc = "Register `SCUE5C` writer"]
pub type W = crate::W<Scue5cSpec>;
#[doc = "Field `SCUREGLOCKB80` reader - SCU_REG_LOCK_B80"]
pub type Scureglockb80R = crate::BitReader;
#[doc = "Field `SCUREGLOCKB80` writer - SCU_REG_LOCK_B80"]
pub type Scureglockb80W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_LOCK_B80"]
    #[inline(always)]
    pub fn scureglockb80(&self) -> Scureglockb80R {
        Scureglockb80R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_LOCK_B80"]
    #[inline(always)]
    pub fn scureglockb80(&mut self) -> Scureglockb80W<Scue5cSpec> {
        Scureglockb80W::new(self, 0)
    }
}
#[doc = "Write Protection 23 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scue5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scue5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scue5cSpec;
impl crate::RegisterSpec for Scue5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scue5c::R`](R) reader structure"]
impl crate::Readable for Scue5cSpec {}
#[doc = "`write(|w| ..)` method takes [`scue5c::W`](W) writer structure"]
impl crate::Writable for Scue5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUE5C to value 0"]
impl crate::Resettable for Scue5cSpec {}
