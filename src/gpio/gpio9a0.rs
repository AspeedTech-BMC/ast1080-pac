#[doc = "Register `GPIO9A0` reader"]
pub type R = crate::R<Gpio9a0Spec>;
#[doc = "Register `GPIO9A0` writer"]
pub type W = crate::W<Gpio9a0Spec>;
#[doc = "Field `GPIO144ReadPrivilegeOfMaster` reader - GPIO144 Read Privilege of Master"]
pub type Gpio144readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO144ReadPrivilegeOfMaster` writer - GPIO144 Read Privilege of Master"]
pub type Gpio144readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO145ReadPrivilegeOfMaster` reader - GPIO145 Read Privilege of Master"]
pub type Gpio145readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO145ReadPrivilegeOfMaster` writer - GPIO145 Read Privilege of Master"]
pub type Gpio145readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO146ReadPrivilegeOfMaster` reader - GPIO146 Read Privilege of Master"]
pub type Gpio146readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO146ReadPrivilegeOfMaster` writer - GPIO146 Read Privilege of Master"]
pub type Gpio146readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GPIO147ReadPrivilegeOfMaster` reader - GPIO147 Read Privilege of Master"]
pub type Gpio147readPrivilegeOfMasterR = crate::FieldReader;
#[doc = "Field `GPIO147ReadPrivilegeOfMaster` writer - GPIO147 Read Privilege of Master"]
pub type Gpio147readPrivilegeOfMasterW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - GPIO144 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio144read_privilege_of_master(&self) -> Gpio144readPrivilegeOfMasterR {
        Gpio144readPrivilegeOfMasterR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - GPIO145 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio145read_privilege_of_master(&self) -> Gpio145readPrivilegeOfMasterR {
        Gpio145readPrivilegeOfMasterR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - GPIO146 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio146read_privilege_of_master(&self) -> Gpio146readPrivilegeOfMasterR {
        Gpio146readPrivilegeOfMasterR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - GPIO147 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio147read_privilege_of_master(&self) -> Gpio147readPrivilegeOfMasterR {
        Gpio147readPrivilegeOfMasterR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO144 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio144read_privilege_of_master(
        &mut self,
    ) -> Gpio144readPrivilegeOfMasterW<Gpio9a0Spec> {
        Gpio144readPrivilegeOfMasterW::new(self, 0)
    }
    #[doc = "Bits 8:15 - GPIO145 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio145read_privilege_of_master(
        &mut self,
    ) -> Gpio145readPrivilegeOfMasterW<Gpio9a0Spec> {
        Gpio145readPrivilegeOfMasterW::new(self, 8)
    }
    #[doc = "Bits 16:23 - GPIO146 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio146read_privilege_of_master(
        &mut self,
    ) -> Gpio146readPrivilegeOfMasterW<Gpio9a0Spec> {
        Gpio146readPrivilegeOfMasterW::new(self, 16)
    }
    #[doc = "Bits 24:31 - GPIO147 Read Privilege of Master"]
    #[inline(always)]
    pub fn gpio147read_privilege_of_master(
        &mut self,
    ) -> Gpio147readPrivilegeOfMasterW<Gpio9a0Spec> {
        Gpio147readPrivilegeOfMasterW::new(self, 24)
    }
}
#[doc = "GPIO Read Privilege Control Register \\#36\n\nYou can [`read`](crate::Reg::read) this register and get [`gpio9a0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gpio9a0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio9a0Spec;
impl crate::RegisterSpec for Gpio9a0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio9a0::R`](R) reader structure"]
impl crate::Readable for Gpio9a0Spec {}
#[doc = "`write(|w| ..)` method takes [`gpio9a0::W`](W) writer structure"]
impl crate::Writable for Gpio9a0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GPIO9A0 to value 0xffff_ffff"]
impl crate::Resettable for Gpio9a0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
