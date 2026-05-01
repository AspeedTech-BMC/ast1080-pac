#[doc = "Register `SCUD5C` reader"]
pub type R = crate::R<Scud5cSpec>;
#[doc = "Register `SCUD5C` writer"]
pub type W = crate::W<Scud5cSpec>;
#[doc = "Field `SCUREGSEC3B80` reader - SCU_REG_SEC3_B80"]
pub type Scuregsec3b80R = crate::BitReader;
#[doc = "Field `SCUREGSEC3B80` writer - SCU_REG_SEC3_B80"]
pub type Scuregsec3b80W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC3_B80"]
    #[inline(always)]
    pub fn scuregsec3b80(&self) -> Scuregsec3b80R {
        Scuregsec3b80R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC3_B80"]
    #[inline(always)]
    pub fn scuregsec3b80(&mut self) -> Scuregsec3b80W<Scud5cSpec> {
        Scuregsec3b80W::new(self, 0)
    }
}
#[doc = "Secure3 Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scud5c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scud5c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scud5cSpec;
impl crate::RegisterSpec for Scud5cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scud5c::R`](R) reader structure"]
impl crate::Readable for Scud5cSpec {}
#[doc = "`write(|w| ..)` method takes [`scud5c::W`](W) writer structure"]
impl crate::Writable for Scud5cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUD5C to value 0"]
impl crate::Resettable for Scud5cSpec {}
