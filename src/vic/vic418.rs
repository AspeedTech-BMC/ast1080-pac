#[doc = "Register `VIC418` reader"]
pub type R = crate::R<Vic418Spec>;
#[doc = "Register `VIC418` writer"]
pub type W = crate::W<Vic418Spec>;
#[doc = "Field `VICSIRQCSEL36` reader - VIC_SIRQ_CSEL3_6"]
pub type Vicsirqcsel36R = crate::FieldReader<u16>;
#[doc = "Field `VICSIRQCSEL36` writer - VIC_SIRQ_CSEL3_6"]
pub type Vicsirqcsel36W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VIC_SIRQ_CSEL3_6"]
    #[inline(always)]
    pub fn vicsirqcsel36(&self) -> Vicsirqcsel36R {
        Vicsirqcsel36R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VIC_SIRQ_CSEL3_6"]
    #[inline(always)]
    pub fn vicsirqcsel36(&mut self) -> Vicsirqcsel36W<Vic418Spec> {
        Vicsirqcsel36W::new(self, 0)
    }
}
#[doc = "Int Routing Select3 6\n\nYou can [`read`](crate::Reg::read) this register and get [`vic418::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vic418::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Vic418Spec;
impl crate::RegisterSpec for Vic418Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vic418::R`](R) reader structure"]
impl crate::Readable for Vic418Spec {}
#[doc = "`write(|w| ..)` method takes [`vic418::W`](W) writer structure"]
impl crate::Writable for Vic418Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VIC418 to value 0"]
impl crate::Resettable for Vic418Spec {}
