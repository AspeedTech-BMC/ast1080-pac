#[doc = "Register `GPIO9D0` reader"]
pub type R = crate::R<Gpio9d0Spec>;
#[doc = "Register `GPIO9D0` writer"]
pub type W = crate::W<Gpio9d0Spec>;
#[doc = "Field `GPIO192ReadPrivilegeOfMaster` reader - GPIO192 Read Privilege of Master"]
pub type Gpio192readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO192ReadPrivilegeOfMaster` writer - GPIO192 Read Privilege of Master"]
pub type Gpio192readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO193ReadPrivilegeOfMaster` reader - GPIO193 Read Privilege of Master"]
pub type Gpio193readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO193ReadPrivilegeOfMaster` writer - GPIO193 Read Privilege of Master"]
pub type Gpio193readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved1` writer - Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO192 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio192read_privilege_of_master(&self) -> Gpio192readPrivilegeOfMasterR {
        Gpio192readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO193 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio193read_privilege_of_master(&self) -> Gpio193readPrivilegeOfMasterR {
        Gpio193readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO192 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio192read_privilege_of_master(
        &mut self,
    ) -> Gpio192readPrivilegeOfMasterW<Gpio9d0Spec> {
        Gpio192readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO193 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio193read_privilege_of_master(
        &mut self,
    ) -> Gpio193readPrivilegeOfMasterW<Gpio9d0Spec> {
        Gpio193readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&mut self) -> Reserved1W<Gpio9d0Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#48\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9d0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9d0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9d0Spec;
impl crate::RegisterSpec for Gpio9d0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9d0::R`](R) reader structure"]
impl crate::Readable for Gpio9d0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9d0::W`](W) writer structure"]
impl crate::Writable for Gpio9d0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9D0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9d0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
