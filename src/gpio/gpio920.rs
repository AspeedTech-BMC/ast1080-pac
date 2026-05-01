#[doc = "Register `GPIO920` reader"]
pub type R = crate::R<Gpio920Spec>;
#[doc = "Register `GPIO920` writer"]
pub type W = crate::W<Gpio920Spec>;
#[doc = "Field `GPIO016ReadPrivilegeOfMaster` reader - GPIO016 Read Privilege of Master"]
pub type Gpio016readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO016ReadPrivilegeOfMaster` writer - GPIO016 Read Privilege of Master"]
pub type Gpio016readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO017ReadPrivilegeOfMaster` reader - GPIO017 Read Privilege of Master"]
pub type Gpio017readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO017ReadPrivilegeOfMaster` writer - GPIO017 Read Privilege of Master"]
pub type Gpio017readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO018ReadPrivilegeOfMaster` reader - GPIO018 Read Privilege of Master"]
pub type Gpio018readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO018ReadPrivilegeOfMaster` writer - GPIO018 Read Privilege of Master"]
pub type Gpio018readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO019ReadPrivilegeOfMaster` reader - GPIO019 Read Privilege of Master"]
pub type Gpio019readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO019ReadPrivilegeOfMaster` writer - GPIO019 Read Privilege of Master"]
pub type Gpio019readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO016 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio016read_privilege_of_master(&self) -> Gpio016readPrivilegeOfMasterR {
        Gpio016readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO017 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio017read_privilege_of_master(&self) -> Gpio017readPrivilegeOfMasterR {
        Gpio017readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO018 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio018read_privilege_of_master(&self) -> Gpio018readPrivilegeOfMasterR {
        Gpio018readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO019 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio019read_privilege_of_master(&self) -> Gpio019readPrivilegeOfMasterR {
        Gpio019readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO016 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio016read_privilege_of_master(
        &mut self,
    ) -> Gpio016readPrivilegeOfMasterW<Gpio920Spec> {
        Gpio016readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO017 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio017read_privilege_of_master(
        &mut self,
    ) -> Gpio017readPrivilegeOfMasterW<Gpio920Spec> {
        Gpio017readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO018 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio018read_privilege_of_master(
        &mut self,
    ) -> Gpio018readPrivilegeOfMasterW<Gpio920Spec> {
        Gpio018readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO019 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio019read_privilege_of_master(
        &mut self,
    ) -> Gpio019readPrivilegeOfMasterW<Gpio920Spec> {
        Gpio019readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#4\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio920::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio920::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio920Spec;
impl crate::RegisterSpec for Gpio920Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio920::R`](R) reader structure"]
impl crate::Readable for Gpio920Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio920::W`](W) writer structure"]
impl crate::Writable for Gpio920Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO920 to value 0xffff_ffff"]
impl crate::Resettable for Gpio920Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
