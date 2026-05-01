#[doc = "Register `SCU98C` reader"]
pub type R = crate::R<Scu98cSpec>;
#[doc = "Register `SCU98C` writer"]
pub type W = crate::W<Scu98cSpec>;
#[doc = "Field `SCUEFUSEPTN` reader - SCU_EFUSE_PTN"]
pub type ScuefuseptnR = crate::FieldReader<u32>;
#[doc = "Field `SCUEFUSEPTN` writer - SCU_EFUSE_PTN"]
pub type ScuefuseptnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - SCU_EFUSE_PTN"]
    #[inline(always)]
    pub fn scuefuseptn(&self) -> ScuefuseptnR {
        ScuefuseptnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SCU_EFUSE_PTN"]
    #[inline(always)]
    pub fn scuefuseptn(&mut self) -> ScuefuseptnW<Scu98cSpec> {
        ScuefuseptnW::new(self, 0)
    }
}
#[doc = "EFUSE Program Pattern Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scu98c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scu98c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scu98cSpec;
impl crate::RegisterSpec for Scu98cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scu98c::R`](R) reader structure"]
impl crate::Readable for Scu98cSpec {}
#[doc = "`write(|w| ..)` method takes [`scu98c::W`](W) writer structure"]
impl crate::Writable for Scu98cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCU98C to value 0"]
impl crate::Resettable for Scu98cSpec {}
