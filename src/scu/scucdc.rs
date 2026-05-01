#[doc = "Register `SCUCDC` reader"]
pub type R = crate::R<ScucdcSpec>;
#[doc = "Register `SCUCDC` writer"]
pub type W = crate::W<ScucdcSpec>;
#[doc = "Field `SCUREGSEC2B80` reader - SCU_REG_SEC2_B80"]
pub type Scuregsec2b80R = crate::BitReader;
#[doc = "Field `SCUREGSEC2B80` writer - SCU_REG_SEC2_B80"]
pub type Scuregsec2b80W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SCU_REG_SEC2_B80"]
    #[inline(always)]
    pub fn scuregsec2b80(&self) -> Scuregsec2b80R {
        Scuregsec2b80R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SCU_REG_SEC2_B80"]
    #[inline(always)]
    pub fn scuregsec2b80(&mut self) -> Scuregsec2b80W<ScucdcSpec> {
        Scuregsec2b80W::new(self, 0)
    }
}
#[doc = "Secure2 Control 24 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scucdc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scucdc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScucdcSpec;
impl crate::RegisterSpec for ScucdcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scucdc::R`](R) reader structure"]
impl crate::Readable for ScucdcSpec {}
#[doc = "`write(|w| ..)` method takes [`scucdc::W`](W) writer structure"]
impl crate::Writable for ScucdcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCUCDC to value 0"]
impl crate::Resettable for ScucdcSpec {}
