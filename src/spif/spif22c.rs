#[doc = "Register `SPIF22C` reader"]
pub type R = crate::R<Spif22cSpec>;
#[doc = "Register `SPIF22C` writer"]
pub type W = crate::W<Spif22cSpec>;
#[doc = "Field `ADDRLBND11` reader - ADDR_LBND11"]
pub type Addrlbnd11R = crate::FieldReader<u16>;
#[doc = "Field `ADDRLBND11` writer - ADDR_LBND11"]
pub type Addrlbnd11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ADDRUBND11` reader - ADDR_UBND11"]
pub type Addrubnd11R = crate::FieldReader<u16>;
#[doc = "Field `ADDRUBND11` writer - ADDR_UBND11"]
pub type Addrubnd11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - ADDR_LBND11"]
    #[inline(always)]
    pub fn addrlbnd11(&self) -> Addrlbnd11R {
        Addrlbnd11R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - ADDR_UBND11"]
    #[inline(always)]
    pub fn addrubnd11(&self) -> Addrubnd11R {
        Addrubnd11R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - ADDR_LBND11"]
    #[inline(always)]
    pub fn addrlbnd11(&mut self) -> Addrlbnd11W<Spif22cSpec> {
        Addrlbnd11W::new(self, 0)
    }
    #[doc = "Bits 16:31 - ADDR_UBND11"]
    #[inline(always)]
    pub fn addrubnd11(&mut self) -> Addrubnd11W<Spif22cSpec> {
        Addrubnd11W::new(self, 16)
    }
}
#[doc = "SPIF\\_ADDRBND11\n\nYou can [`read`](crate::Reg::read) this register and get [`spif22c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spif22c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spif22cSpec;
impl crate::RegisterSpec for Spif22cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spif22c::R`](R) reader structure"]
impl crate::Readable for Spif22cSpec {}
#[doc = "`write(|w| ..)` method takes [`spif22c::W`](W) writer structure"]
impl crate::Writable for Spif22cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPIF22C to value 0"]
impl crate::Resettable for Spif22cSpec {}
