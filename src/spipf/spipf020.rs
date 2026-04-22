#[doc = "Register `SPIPF020` reader"]
pub type R = crate::R<Spipf020Spec>;
#[doc = "Register `SPIPF020` writer"]
pub type W = crate::W<Spipf020Spec>;
#[doc = "Field `Base` reader - Base"]
pub type BaseR = crate::FieldReader<u16>;
#[doc = "Field `Base` writer - Base"]
pub type BaseW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `End` reader - End"]
pub type EndR = crate::FieldReader<u16>;
#[doc = "Field `End` writer - End"]
pub type EndW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Base"]
    #[inline(always)]
    pub fn base(&self) -> BaseR {
        BaseR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - End"]
    #[inline(always)]
    pub fn end(&self) -> EndR {
        EndR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Base"]
    #[inline(always)]
    pub fn base(&mut self) -> BaseW<Spipf020Spec> {
        BaseW::new(self, 0)
    }
    #[doc = "Bits 16:31 - End"]
    #[inline(always)]
    pub fn end(&mut self) -> EndW<Spipf020Spec> {
        EndW::new(self, 16)
    }
}
#[doc = "CS0 range\n\nYou can [`read`](crate::Reg::read) this register and get [`spipf020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spipf020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spipf020Spec;
impl crate::RegisterSpec for Spipf020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spipf020::R`](R) reader structure"]
impl crate::Readable for Spipf020Spec {}
#[doc = "`write(|w| ..)` method takes [`spipf020::W`](W) writer structure"]
impl crate::Writable for Spipf020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIPF020 to value 0"]
impl crate::Resettable for Spipf020Spec {}
