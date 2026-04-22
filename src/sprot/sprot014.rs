#[doc = "Register `SPROT014` reader"]
pub type R = crate::R<Sprot014Spec>;
#[doc = "Register `SPROT014` writer"]
pub type W = crate::W<Sprot014Spec>;
#[doc = "Field `SID04` reader - SID04"]
pub type Sid04R = crate::FieldReader;
#[doc = "Field `SID04` writer - SID04"]
pub type Sid04W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SID05` reader - SID05"]
pub type Sid05R = crate::FieldReader;
#[doc = "Field `SID05` writer - SID05"]
pub type Sid05W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SID06` reader - SID06"]
pub type Sid06R = crate::FieldReader;
#[doc = "Field `SID06` writer - SID06"]
pub type Sid06W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SID07` reader - SID07"]
pub type Sid07R = crate::FieldReader;
#[doc = "Field `SID07` writer - SID07"]
pub type Sid07W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - SID04"]
    #[inline(always)]
    pub fn sid04(&self) -> Sid04R {
        Sid04R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - SID05"]
    #[inline(always)]
    pub fn sid05(&self) -> Sid05R {
        Sid05R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - SID06"]
    #[inline(always)]
    pub fn sid06(&self) -> Sid06R {
        Sid06R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - SID07"]
    #[inline(always)]
    pub fn sid07(&self) -> Sid07R {
        Sid07R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SID04"]
    #[inline(always)]
    pub fn sid04(&mut self) -> Sid04W<Sprot014Spec> {
        Sid04W::new(self, 0)
    }
    #[doc = "Bits 8:15 - SID05"]
    #[inline(always)]
    pub fn sid05(&mut self) -> Sid05W<Sprot014Spec> {
        Sid05W::new(self, 8)
    }
    #[doc = "Bits 16:23 - SID06"]
    #[inline(always)]
    pub fn sid06(&mut self) -> Sid06W<Sprot014Spec> {
        Sid06W::new(self, 16)
    }
    #[doc = "Bits 24:31 - SID07"]
    #[inline(always)]
    pub fn sid07(&mut self) -> Sid07W<Sprot014Spec> {
        Sid07W::new(self, 24)
    }
}
#[doc = "SPROT\\_SIDG1\n\nYou can [`read`](crate::Reg::read) this register and get [`sprot014::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sprot014::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sprot014Spec;
impl crate::RegisterSpec for Sprot014Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sprot014::R`](R) reader structure"]
impl crate::Readable for Sprot014Spec {}
#[doc = "`write(|w| ..)` method takes [`sprot014::W`](W) writer structure"]
impl crate::Writable for Sprot014Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPROT014 to value 0"]
impl crate::Resettable for Sprot014Spec {}
